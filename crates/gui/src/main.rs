use std::{cell::RefCell, rc::Rc};

use anyhow::Context;
use lib::{Effect, Effects, Image};
use log::{info, warn};
use simple_logger::SimpleLogger;
use slint::ComponentHandle;

use crate::{
    effect::effects_to_model, image::image_to_slint, move_direction::move_direction_from_slint,
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

    // temp sample effects
    info!("Loading effects");
    let effects = Effects::load("effects5.json").unwrap();

    // temp sample input image
    info!("Loading input image");
    let original_image = Image::open("sample3.jpg").unwrap();
    let mut working_image = original_image.clone();
    working_image.effects(&effects);

    // make Rc and RefCell from these
    let original_image = Rc::new(original_image);
    let effects = Rc::new(RefCell::new(effects));

    // run app
    let ui = App::new().context("Failed to initiate App")?;

    // input structs
    ui.set_effects(effects_to_model(&effects.borrow()));
    ui.set_original_image(image_to_slint(&original_image));
    ui.set_working_image(image_to_slint(&working_image));

    // TODO: a lot of duplicate code: create app state struct
    // change effect
    let ui_weak = ui.as_weak();
    let new_effects = Rc::clone(&effects);
    let new_image = Rc::clone(&original_image);
    ui.on_effect_changed(move |id, value| {
        new_effects.borrow_mut().change_value(id as usize, value);
        let mut image = new_image.as_ref().clone();
        image.effects(&new_effects.borrow());
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_working_image(image_to_slint(&image));
        }
    });

    // add effect
    let ui_weak = ui.as_weak();
    let new_effects = Rc::clone(&effects);
    let new_image = Rc::clone(&original_image);
    ui.on_effect_added(move |name, value| {
        if let Some(effect) = Effect::try_from_name(name.as_str(), value) {
            new_effects.borrow_mut().push(effect);
            let mut image = new_image.as_ref().clone();
            image.effects(&new_effects.borrow());
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_effects(effects_to_model(&new_effects.borrow()));
                ui.set_working_image(image_to_slint(&image));
            }
        } else {
            warn!("Unknown effect name: '{}'", name);
        }
    });

    // remove effect
    let ui_weak = ui.as_weak();
    let new_effects = Rc::clone(&effects);
    let new_image = Rc::clone(&original_image);
    ui.on_effect_removed(move |id| {
        new_effects.borrow_mut().remove(id as usize);
        let mut image = new_image.as_ref().clone();
        image.effects(&new_effects.borrow());
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_effects(effects_to_model(&new_effects.borrow()));
            ui.set_working_image(image_to_slint(&image));
        }
    });

    // move effect
    let ui_weak = ui.as_weak();
    let new_effects = Rc::clone(&effects);
    let new_image = Rc::clone(&original_image);
    ui.on_effect_moved(move |id, direction| {
        let direction: lib::MoveDirection = move_direction_from_slint(&direction);
        new_effects
            .borrow_mut()
            .move_effect(id as usize, &direction);
        let mut image = new_image.as_ref().clone();
        image.effects(&new_effects.borrow());
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_effects(effects_to_model(&new_effects.borrow()));
            ui.set_working_image(image_to_slint(&image));
        }
    });

    ui.run().context("Failed to run App")?;

    Ok(())
}
