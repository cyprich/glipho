use std::{cell::RefCell, rc::Rc};

use anyhow::Context;
use lib::{Effect, Image};
use rfd::FileDialog;
use simple_logger::SimpleLogger;
use slint::ComponentHandle;

use crate::{
    effect::effects_to_slint, image::image_to_slint, move_direction::move_direction_from_slint,
};

mod colors;
mod effect;
mod image;
mod move_direction;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    // initiate logger, ignore logs from these libraries
    let ignored = ["tracing", "calloop", "sctk", "zbus", "winit"];
    let mut logger = SimpleLogger::new();
    for l in ignored {
        logger = logger.with_module_level(l, log::LevelFilter::Off);
    }
    logger.init().context("Failed to initiate simple_logger")?;

    // default structs
    let original_image = Rc::new(RefCell::new(None::<lib::Image>));
    let effects = Rc::new(RefCell::new(lib::Effects::default()));

    // run app
    let ui = App::new().context("Failed to initiate App")?;

    // input structs
    ui.set_effects(effects_to_slint(None));
    ui.set_original_image(image_to_slint(None));
    ui.set_working_image(image_to_slint(None));

    // TODO: a lot of duplicate code
    // change effect
    let ui_weak = ui.as_weak();
    let new_image = Rc::clone(&original_image);
    let new_effects = Rc::clone(&effects);
    ui.on_effect_changed(move |id, value| {
        if let Some(image) = new_image.borrow_mut().as_mut() {
            let mut new_effects = new_effects.borrow_mut();
            new_effects.change_value(id as usize, value);

            let mut image = image.clone();
            image.effects(&new_effects);

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_working_image(image_to_slint(Some(&image)));
            }
        }
    });

    // add effect
    let ui_weak = ui.as_weak();
    let new_image = Rc::clone(&original_image);
    let new_effects = Rc::clone(&effects);
    ui.on_effect_added(move |name, value| {
        if let Some(image) = new_image.borrow_mut().as_mut()
            && let Some(effect) = Effect::try_from_name(&name, value)
        {
            let mut new_effects = new_effects.borrow_mut();
            new_effects.push(effect);

            let mut image = image.clone();
            image.effects(&new_effects);

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_effects(effects_to_slint(Some(&new_effects)));
                ui.set_working_image(image_to_slint(Some(&image)));
            }
        }
    });

    // remove effect
    let ui_weak = ui.as_weak();
    let new_image = Rc::clone(&original_image);
    let new_effects = Rc::clone(&effects);
    ui.on_effect_removed(move |id| {
        if let Some(image) = new_image.borrow_mut().as_mut() {
            let mut new_effects = new_effects.borrow_mut();
            new_effects.remove(id as usize);

            let mut image = image.clone();
            image.effects(&new_effects);

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_effects(effects_to_slint(Some(&new_effects)));
                ui.set_working_image(image_to_slint(Some(&image)));
            }
        }
    });

    // move effect
    let ui_weak = ui.as_weak();
    let new_image = Rc::clone(&original_image);
    let new_effects = Rc::clone(&effects);
    ui.on_effect_moved(move |id, direction| {
        if let Some(image) = new_image.borrow_mut().as_mut() {
            let direction: lib::MoveDirection = move_direction_from_slint(&direction);

            let mut new_effects = new_effects.borrow_mut();
            new_effects.move_effect(id as usize, &direction);

            let mut image = image.clone();
            image.effects(&new_effects);

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_effects(effects_to_slint(Some(&new_effects)));
                ui.set_working_image(image_to_slint(Some(&image)));
            }
        }
    });

    // open image
    let ui_weak = ui.as_weak();
    let original_image = Rc::clone(&original_image);
    let effects = Rc::clone(&effects);
    ui.on_open_image(move || {
        let path = FileDialog::new()
            .add_filter("Image", &["png", "jpg"])
            .set_title("Open image")
            .pick_file();

        if let Some(path) = path
            && let Ok(image) = Image::open(path)
        {
            *original_image.borrow_mut() = Some(image.clone());
            let mut working_image = image;

            working_image.effects(&effects.borrow());

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_original_image(image_to_slint(original_image.borrow().as_ref()));
                ui.set_working_image(image_to_slint(Some(&working_image)));
                ui.set_has_image(true);
            }
        }
    });

    ui.run().context("Failed to run App")?;

    Ok(())
}
