use std::fs;

use anyhow::{Context, Error, Result};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use lib::{Image, Layer, Step, Steps};

pub fn run() -> Result<()> {
    println!("Welcome to Rusty Fotos!");

    let mut image: Option<Image> = None;
    let mut steps = Steps::default();
    let mut unsaved_changes = false;

    let theme = ColorfulTheme::default();

    let root_items = vec![
        "Load/Change Image",
        "View Steps",
        "Add Step",
        "Remove Step",
        "Save Steps to file",
        "Load Steps from file",
        "Apply Steps",
        "Exit",
    ];
    let root_selection = Select::with_theme(&theme)
        .with_prompt("Choose your action")
        .items(&root_items);

    let unsaved_confirmation = Confirm::with_theme(&theme)
        .with_prompt("You have unsaved changes, do you wish to continue?");

    loop {
        println!();
        match root_selection.clone().interact().unwrap() {
            0 => match load_image() {
                Ok(val) => image = Some(val),
                Err(val) => {
                    eprintln!("Failed to load image: {}", val);
                    continue;
                }
            },
            1 => {
                if steps.steps.is_empty() {
                    println!("There are no steps");
                } else {
                    steps
                        .steps
                        .iter()
                        .enumerate()
                        .for_each(|(index, step)| println!("  {}. {}", index + 1, step));
                }
            }
            2 => match add_step() {
                Ok(val) => {
                    unsaved_changes = true;
                    steps.push(val);
                }
                Err(val) => {
                    eprintln!("Failed to add step: {}", val)
                }
            },
            3 => match remove_step(&steps) {
                Ok(val) => {
                    unsaved_changes = true;
                    steps.remove(val);
                }
                Err(val) => {
                    eprintln!("Failed to remove step: {}", val);
                    continue;
                }
            },
            4 => {
                if let Err(val) = save_steps(&steps) {
                    eprintln!("Failed to save steps: {}", val)
                } else {
                    unsaved_changes = false;
                }
            }
            5 => {
                match load_steps() {
                    Ok(val) => steps = val,
                    Err(val) => {
                        eprintln!("{}", val);
                        continue;
                    }
                }
                unsaved_changes = false;
            }
            6 => match image.clone() {
                Some(mut image) => {
                    for s in &steps.steps {
                        match s {
                            Step::Layer(layer) => {
                                image.layer(layer);
                            }
                            Step::Save(filename) => {
                                image.save(filename)?;
                            }
                        }
                    }
                }
                None => eprintln!("No image to apply steps to"),
            },
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
            _ => println!("Hey, this is not supposed to happen!"),
        }
    }
}

fn remove_step(steps: &Steps) -> Result<usize> {
    if steps.steps.is_empty() {
        Err(Error::msg("There are no steps"))
    } else {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which step you want to remove?")
            .items(steps.steps_string())
            .interact()
            .context("Error with selecting step to remove")
    }
}

fn save_steps(steps: &Steps) -> Result<()> {
    let filename = Input::<String>::new()
        .with_prompt("Enter filename (.toml extension will be added automatically)")
        .interact()
        .context("Failed to input filename")?;
    let filename = format!("{}.toml", filename);

    let val = steps.to_toml()?;

    fs::write(filename, val).context("Failed to save")?;

    Ok(())
}

fn load_steps() -> Result<Steps> {
    let filename = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter filename (.toml extension will be added automatically)")
        .interact()?;
    let filename = format!("{}.toml", filename);
    let string = fs::read_to_string(filename).context("Failed to read file")?;
    let result = Steps::from_toml(&string).context("Failed to deserialize")?;
    Ok(result)
}

fn load_image() -> Result<Image> {
    let filename = Input::<String>::new()
        .with_prompt("Enter filename")
        .interact()
        .context("Failed to input filename")?;

    Image::from_file(&filename)
}

fn add_step() -> Result<Step> {
    let items = vec!["Layer", "Save"];
    let selector = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of Step do you want to add? (ESC to cancel)")
        .items(&items)
        .interact_opt()?;

    match selector {
        Some(val) => match val {
            0 => Ok(Step::Layer(add_step_layer()?)),
            1 => Ok(Step::Save(add_step_save()?)),
            _ => Err(Error::msg("This was not supposed to happen...")),
        },
        None => Err(Error::msg("User cancelled")),
    }
}

fn add_step_layer() -> Result<Layer> {
    let items = vec!["Brightness", "Wrapped Brightness", "Invert", "Reverse Bits"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What kind of layer do you want to add? (ESC to cancel)")
        .items(&items)
        .interact_opt()?;

    let theme = ColorfulTheme::default();
    let number_input =
        Input::<i16>::with_theme(&theme).with_prompt("Enter the amount (-255 to 255)");

    match selection {
        Some(val) => match val {
            0 | 1 => {
                let num = number_input.interact()?;
                if !(-255..=255).contains(&num) {
                    Err(Error::msg("Invalid range"))
                } else {
                    match val {
                        0 => Ok(Layer::Brightness(num)),
                        1 => Ok(Layer::WrapBrightness(num)),
                        _ => Err(Error::msg("This was not supposed to happen...")),
                    }
                }
            }
            2 => Ok(Layer::Invert),
            3 => Ok(Layer::ReverseBits),
            _ => Err(Error::msg("This was not supposed to happen")),
        },
        None => Err(Error::msg("User cancelled")),
    }
}

fn add_step_save() -> Result<String> {
    let filename = Input::<String>::new()
        .with_prompt("Enter filename")
        .interact()?;
    Ok(filename)
}
