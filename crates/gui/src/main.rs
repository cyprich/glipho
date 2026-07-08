use anyhow::Context;
use slint::ComponentHandle;

slint::include_modules!();
fn main() -> anyhow::Result<()> {
    // let ui = App::new().context("Failed to initiate App")?;
    let ui = Counter::new().context("Failed to initiate App")?;
    let ui_handle = ui.as_weak();

    {
        // increase
        let ui_handle = ui_handle.clone();
        ui.on_increase(move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        });
    }
    {
        // decrease
        let ui_handle = ui_handle.clone();
        ui.on_decrease(move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() - 1);
        });
    }
    {
        // reset
        let ui_handle = ui_handle.clone();
        ui.on_reset(move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(0);
        });
    }
    {
        // change text value
        let ui_handle = ui_handle.clone();
        ui.on_change_textvalue(move || {
            let ui = ui_handle.unwrap();
            ui.set_textvalue(ui.get_textinput());
        });
    }

    ui.run().context("Failed to run App")?;

    Ok(())
}
