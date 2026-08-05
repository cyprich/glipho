use std::{cell::RefCell, rc::Rc};

use anyhow::Context;
use lib::Effect;
use rfd::FileDialog;
use simple_logger::SimpleLogger;
use slint::ComponentHandle;

use crate::{
    effect::effects_to_slint, image::image_to_slint, move_direction::move_direction_from_slint,
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
    let state = Rc::new(RefCell::new(AppState::new(&ui)));

    // input structs
    ui.set_effects(effects_to_slint(None));
    ui.set_original_image(image_to_slint(None));
    ui.set_working_image(image_to_slint(None));

    {
        // change effect
        let state = Rc::clone(&state);
        ui.on_effect_changed(move |id, value| {
            let mut state = state.borrow_mut();
            state.effects.change_value(id as usize, value);
            state.redraw_working_image(true);
        });
    }
    {
        // add effect
        let state = Rc::clone(&state);
        ui.on_effect_added(move |name, value| {
            if let Some(effect) = Effect::try_from_name(&name, value) {
                let mut state = state.borrow_mut();
                state.effects.push(effect);
                state.redraw_working_image(true);
            }
        });
    }
    {
        // remove effect
        let state = Rc::clone(&state);
        ui.on_effect_removed(move |id| {
            let mut state = state.borrow_mut();
            state.effects.remove(id as usize);
            state.redraw_working_image(true);
        });
    }
    {
        // move effect
        let state = Rc::clone(&state);
        ui.on_effect_moved(move |id, direction| {
            let mut state = state.borrow_mut();
            let direction = move_direction_from_slint(&direction);
            state.effects.move_effect(id as usize, &direction);
            state.redraw_working_image(true);
        });
    }

    {
        let state = Rc::clone(&state);
        ui.on_open_image(move || {
            let path = FileDialog::new()
                .add_filter("Image", &["png", "jpg"])
                .set_title("Open image")
                .pick_file();

            let mut state = state.borrow_mut();
            state.load_image(path);
        });
    }

    ui.run().context("Failed to run App")?;

    Ok(())
}
