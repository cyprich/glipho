use anyhow::Context;
use slint::ComponentHandle;

slint::include_modules!();
fn main() -> anyhow::Result<()> {
    // let ui = App::new().context("Failed to initiate App")?;
    let ui = Counter::new().context("Failed to initiate App")?;

    ui.on_increase({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_decrease({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() - 1);
        }
    });

    ui.on_reset({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(0);
        }
    });

    ui.on_change_textvalue({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_textvalue(ui.get_textinput());
        }
    });

    ui.run().context("Failed to run App")?;

    Ok(())
}
