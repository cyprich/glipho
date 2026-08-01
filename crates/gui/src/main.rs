use anyhow::Context;
use lib::Effects;
use slint::ComponentHandle;

use crate::effect::effects_to_model;

mod colors;
mod effect;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    // TODO: temp
    let effects = Effects::load("effects5.json").unwrap();

    let ui = App::new().context("Failed to initiate App")?;

    ui.set_effects(effects_to_model(effects));

    ui.run().context("Failed to run App")?;

    Ok(())
}
