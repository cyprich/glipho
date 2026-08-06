use lib::Effects;
use slint::ComponentHandle;

use crate::App;

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
}
