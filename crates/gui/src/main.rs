use anyhow::Context;
use slint::ComponentHandle;

mod ui;

fn main() -> anyhow::Result<()> {
    ui::App::new()
        .context("Failed to initiate new App")?
        .run()
        .context("Failed to run App")?;

    Ok(())
}
