use anyhow::{Context, Error, Result};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use lib::{Effect, Effects, Image};
use log::{error, warn};

pub fn run(effects: Option<Effects>, image: Option<Image>) -> Result<()> {
    let mut original_image = image;

    let mut effects = effects.unwrap_or_default();
    let mut unsaved_changes = false;

    let theme = ColorfulTheme::default();

    let root_items = vec![
        "Load/Change Image",
        "View Effects",
        "Add Effect",
        "Remove Effect",
        "Save Effects to file",
        "Load Effects from file",
        "Apply Effects",
        "Exit",
    ];
    let root_selection = Select::with_theme(&theme)
        .with_prompt("Choose your action")
        .items(&root_items);

    let unsaved_confirmation = Confirm::with_theme(&theme)
        .with_prompt("You have unsaved changes, do you wish to continue?");

    loop {
        println!();
        // choose action
        match root_selection.clone().interact()? {
            // load/change image
            0 => match load_image() {
                Ok(val) => original_image = Some(val),
                Err(val) => {
                    error!("Failed to load image: {}", val);
                    continue;
                }
            },
            // view effects
            1 => {
                if effects.inner.is_empty() {
                    println!("There are no effects");
                } else {
                    effects
                        .inner
                        .iter()
                        .enumerate()
                        .for_each(|(index, effect)| println!("  {}. {}", index + 1, effect));
                }
            }
            // add effect
            2 => match add_effect() {
                Ok(val) => {
                    unsaved_changes = true;
                    effects.inner.push(val);
                }
                Err(val) => {
                    error!("Failed to add effect: {}", val)
                }
            },
            // remove effect
            3 => match remove_effect(&effects) {
                Ok(val) => {
                    unsaved_changes = true;
                    effects.inner.remove(val);
                }
                Err(val) => {
                    error!("Failed to remove effect: {}", val);
                    continue;
                }
            },
            // save effects to file
            4 => {
                if let Err(val) = save_effects(&effects) {
                    error!("Failed to save effects: {}", val)
                } else {
                    unsaved_changes = false;
                }
            }
            // load effects from file
            5 => {
                match load_effects() {
                    Ok(val) => effects = val,
                    Err(val) => {
                        error!("{}", val);
                        continue;
                    }
                }
                unsaved_changes = false;
            }
            // apply effects
            6 => {
                let mut working_image = original_image.clone();
                if let Some(image) = working_image.as_mut() {
                    image.effects(&effects);
                } else {
                    error!("No image to apply effects to");
                }
            }
            // exit
            7 => {
                if unsaved_changes {
                    if unsaved_confirmation
                        .clone()
                        .interact()
                        .context("Failed to interact with confirmation dialog")?
                    {
                        return Ok(());
                    }
                } else {
                    return Ok(());
                }
            }
            _ => warn!("Hey, this is not supposed to happen!"),
        }
    }
}

fn remove_effect(effects: &Effects) -> Result<usize> {
    if effects.inner.is_empty() {
        Err(Error::msg("There are no effects"))
    } else {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which effect you want to remove?")
            .items(effects.inner.iter().map(|e| e.to_string()))
            .interact()
            .context("Error with selecting effect to remove")
    }
}

fn save_effects(effects: &Effects) -> Result<()> {
    let path = Input::<String>::new()
        .with_prompt("Enter path")
        .interact()
        .context("Failed to input filename")?;

    effects.save(path)?;

    Ok(())
}

fn load_effects() -> Result<Effects> {
    let path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Image path")
        .interact()?;
    let result = Effects::load(&path)?;
    Ok(result)
}

fn load_image() -> Result<Image> {
    let filename = Input::<String>::new()
        .with_prompt("Enter filename")
        .interact()
        .context("Failed to input filename")?;

    Image::open(&filename)
}

fn add_effect() -> Result<Effect> {
    let items = vec![
        "Brightness",
        "Wrapped Brightness",
        "Invert",
        "Reverse Bits",
        "Min",
        "Max",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What kind of layer do you want to add? (ESC to cancel)")
        .items(&items)
        .interact_opt()?;

    let theme = ColorfulTheme::default();
    let number_input = Input::<i16>::with_theme(&theme);

    match selection {
        Some(val) => match val {
            0 | 1 => {
                let num = number_input
                    .with_prompt("Enter the amount (-255 to 255)")
                    .interact()?;
                if !(-255..=255).contains(&num) {
                    Err(Error::msg("Value out of range"))
                } else {
                    match val {
                        0 => Ok(Effect::Brightness(num)),
                        1 => Ok(Effect::WrapBrightness(num)),
                        _ => Err(Error::msg("This was not supposed to happen...")),
                    }
                }
            }
            2 => Ok(Effect::Invert),
            3 => Ok(Effect::ReverseBits),
            4 | 5 => {
                let num = number_input
                    .with_prompt("Enter the amount (0 to 255)")
                    .interact()?;
                if !(0..255).contains(&num) {
                    Err(Error::msg("Value out of range"))
                } else {
                    match val {
                        4 => Ok(Effect::Min(num as u8)),
                        5 => Ok(Effect::Max(num as u8)),
                        _ => Err(Error::msg("This was not supposed to happen...")),
                    }
                }
            }
            _ => Err(Error::msg("This was not supposed to happen")),
        },
        None => Err(Error::msg("User cancelled")),
    }
}
