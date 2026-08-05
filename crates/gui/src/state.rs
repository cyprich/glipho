use std::path::Path;

use lib::{Effects, Image};
use log::error;
use slint::ComponentHandle;

use crate::{App, effect::effects_to_slint, image::image_to_slint};

pub struct AppState {
    pub ui_weak: slint::Weak<App>,
    pub original_image: Option<lib::Image>,
    pub effects: lib::Effects,
}

impl AppState {
    pub fn new(ui: &App) -> Self {
        Self {
            ui_weak: ui.as_weak(),
            original_image: None,
            effects: Effects::default(),
        }
    }

    pub fn redraw_effects(&mut self) {
        if let Some(ui) = self.ui_weak.upgrade() {
            ui.set_effects(effects_to_slint(Some(&self.effects)));
        }
    }

    pub fn redraw_working_image(&mut self, redraw_effects: bool) {
        let Some(image) = &self.original_image else {
            return;
        };

        let mut image = image.clone();
        image.effects(&self.effects);

        if let Some(ui) = self.ui_weak.upgrade() {
            ui.set_working_image(image_to_slint(Some(&image)));

            if redraw_effects {
                ui.set_effects(effects_to_slint(Some(&self.effects)));
            }
        }
    }

    pub fn load_image(&mut self, path: Option<impl AsRef<Path>>) {
        let Some(path) = path else {
            return;
        };

        let path = path.as_ref();

        let image = match Image::open(path) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed opening '{}': {}", path.to_string_lossy(), e);
                return;
            }
        };

        self.original_image = Some(image);

        if let Some(ui) = self.ui_weak.upgrade() {
            ui.set_original_image(image_to_slint(self.original_image.as_ref()));
            ui.set_has_image(true);
        }

        self.redraw_working_image(true);
    }
}
