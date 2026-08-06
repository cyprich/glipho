use std::sync::{Arc, Mutex};

use anyhow::Context;
use lib::Effect;
use rfd::FileDialog;
use simple_logger::SimpleLogger;
use slint::ComponentHandle;

use crate::{
    effect::effects_to_slint,
    image::{image_to_pixel_buffer, image_to_slint},
    move_direction::move_direction_from_slint,
    state::AppState,
};

mod colors;
mod effect;
mod image;
mod move_direction;
mod state;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    // initiate logger, ignore logs from these libraries
    let ignored = ["tracing", "calloop", "sctk", "zbus", "winit"];
    let mut logger = SimpleLogger::new();
    for l in ignored {
        logger = logger.with_module_level(l, log::LevelFilter::Off);
    }
    logger.init().context("Failed to initiate simple_logger")?;

    // run app
    let ui = App::new().context("Failed to initiate App")?;
    // let state = Rc::new(RefCell::new(AppState::new(&ui)));
    let state = Arc::new(Mutex::new(AppState::new(&ui)));

    // input structs
    ui.set_effects(effects_to_slint(None));
    ui.set_original_image(image_to_slint(None));
    ui.set_working_image(image_to_slint(None));

    // callbacks / actions
    {
        // change effect
        let state = Arc::clone(&state);
        ui.on_effect_changed(move |id, new_value| {
            {
                let mut state = state.lock().unwrap();
                state.effects.change_value(id as usize, new_value);
            }
            process_effects_async(Arc::clone(&state));
        });
    }

    {
        // add effect
        let state = Arc::clone(&state);
        ui.on_effect_added(move |name, value| {
            {
                let mut state = state.lock().unwrap();
                if let Some(effect) = Effect::try_from_name(&name, value) {
                    state.effects.push(effect);
                }
            }
            process_effects_async(Arc::clone(&state));
        });
    }

    {
        // remove effect
        let state = Arc::clone(&state);
        ui.on_effect_removed(move |id| {
            {
                let mut state = state.lock().unwrap();
                state.effects.remove(id as usize);
            }
            process_effects_async(Arc::clone(&state));
        });
    }

    {
        // move effect
        let state = Arc::clone(&state);
        ui.on_effect_moved(move |id, direction| {
            {
                let mut state = state.lock().unwrap();
                let direction = move_direction_from_slint(&direction);
                state.effects.move_effect(id as usize, &direction);
            }
            process_effects_async(Arc::clone(&state));
        });
    }

    {
        // open image
        let state = Arc::clone(&state);

        ui.on_open_image(move || {
            // set is loading
            {
                let state = state.lock().unwrap();
                if let Some(ui) = state.ui_weak.upgrade() {
                    ui.set_is_loading(true);
                }
            }

            let state = Arc::clone(&state);

            std::thread::spawn(move || {
                // pick file
                let path = FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg"])
                    .set_title("Open image")
                    .pick_file();

                if let Some(path) = path
                    && let Ok(image) = lib::Image::open(path)
                {
                    // set original image, get ui weak
                    let ui_weak = {
                        let mut state = state.lock().unwrap();
                        state.original_image = Some(image.clone());
                        state.ui_weak.clone()
                    };

                    // update stuff
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_weak.upgrade() {
                            ui.set_original_image(image_to_slint(Some(&image)));
                            ui.set_has_image(true);
                        }
                    });

                    process_effects_async(state);
                } else {
                    if let Some(ui) = state.lock().unwrap().ui_weak.upgrade() {
                        ui.set_is_loading(false);
                    }
                }
            });
        });
    }

    ui.run().context("Failed to run App")?;

    Ok(())
}

fn process_effects_async(state: Arc<Mutex<AppState>>) {
    {
        // update effects, set working image to loading
        let state = state.lock().unwrap();
        if let Some(ui) = state.ui_weak.upgrade() {
            ui.set_effects(effects_to_slint(Some(&state.effects)));
            ui.set_is_loading(true);
        }
    }

    std::thread::spawn(move || {
        // get these from state
        let (ui_weak, original_image, effects) = {
            let state = state.lock().unwrap();
            (
                state.ui_weak.clone(),
                state.original_image.clone(),
                state.effects.clone(),
            )
        };

        // clone to working image, so original will stay untouched
        let working_image = original_image.clone();

        if let Some(mut image) = working_image {
            // apply effects
            image.effects(&effects);
            let pixel_buffer = image_to_pixel_buffer(&image);

            // update ui
            let _ = slint::invoke_from_event_loop(move || {
                let image = slint::Image::from_rgba8(pixel_buffer);

                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_working_image(image);
                    ui.set_is_loading(false);
                }
            });
        }
    });
}
