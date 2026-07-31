use std::{fmt::format, fs};

use anyhow::{Context, Error, Result};
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use lib::{Effect, Effects, Image};
use log::{error, info, warn};

use crate::{action::Action, screen::Screen};

pub struct Manual {
    input: Option<Image>,
    output: Option<String>,
    effects: Option<Effects>,
    screen: Screen,
}

impl Manual {
    pub fn new(input: Option<Image>, output: Option<String>, effects: Option<Effects>) -> Self {
        Self {
            input,
            output,
            effects,
            screen: Screen::Main,
        }
    }

    pub fn run(mut self) -> Result<()> {
        loop {
            let action = match self.screen {
                Screen::Main => self.render_main(),
                Screen::Image => self.render_image(),
                Screen::ChangeInput => self.change_input_or_output(true),
                Screen::ChangeOutput => self.change_input_or_output(false),
                Screen::Effects => self.render_effects(),
                Screen::ViewEffects => self.view_effects(),
                Screen::AddEffect => todo!(),
                Screen::DeleteEffects => self.delete_effects(),
                Screen::Apply => todo!(),
            }?;

            match (action, self.screen) {
                (Action::Change(screen), _) => self.screen = screen,
                (Action::Exit, Screen::Main) => break,
                (Action::Exit, Screen::Image) => self.screen = Screen::Main,
                (Action::Exit, Screen::ChangeInput) => self.screen = Screen::Image,
                (Action::Exit, Screen::ChangeOutput) => self.screen = Screen::Image,
                (Action::Exit, Screen::Effects) => self.screen = Screen::Main,
                (Action::Exit, Screen::ViewEffects) => self.screen = Screen::Effects,
                (Action::Exit, Screen::AddEffect) => self.screen = Screen::Effects,
                (Action::Exit, Screen::DeleteEffects) => self.screen = Screen::Effects,
                (Action::Exit, Screen::Apply) => todo!(),
            }
        }
        Ok(())
    }

    fn render_main(&mut self) -> Result<Action> {
        let image = format!(
            "Image (input: {}, output: {})",
            match &self.input {
                Some(val) => format!("'{}'", val.path().to_string_lossy()),
                None => "None".to_string(),
            },
            match &self.output {
                Some(val) => format!("'{}'", val),
                None => "None".to_string(),
            }
        );
        let effects = format!(
            "Effects ({} defined)",
            match &self.effects {
                Some(val) => val.inner.len(),
                None => 0,
            }
        );
        let apply = "Apply effects and export image".to_string();
        let exit = "Exit".to_string();

        let items = [image, effects, apply, exit];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Main menu")
            .items(items)
            .interact()?;

        match selection {
            0 => Ok(Action::Change(Screen::Image)),
            1 => Ok(Action::Change(Screen::Effects)),
            2 => Ok(Action::Change(Screen::Apply)),
            3 => Ok(Action::Exit),
            _ => Err(anyhow::Error::msg("Invalid selection index")),
        }
    }

    fn render_image(&mut self) -> Result<Action> {
        let input = format!(
            "Change input image{}",
            match &self.input {
                Some(val) => format!(" ({})", val.path().to_string_lossy()),
                None => String::default(),
            }
        );
        let output = format!(
            "Change output image{}",
            match &self.output {
                Some(val) => format!(" ({})", val),
                None => String::default(),
            }
        );
        let cancel = "Cancel".to_string();
        let items = [input, output, cancel];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Image menu")
            .items(&items)
            .interact()?;

        match selection {
            0 => Ok(Action::Change(Screen::ChangeInput)),
            1 => Ok(Action::Change(Screen::ChangeOutput)),
            2 => Ok(Action::Exit),
            _ => Err(anyhow::Error::msg("Invalid selection index")),
        }
    }

    fn render_effects(&mut self) -> Result<Action> {
        let mut items = vec![];

        if let Some(val) = &self.effects {
            let val = format!("View effects ({} defined)", val.inner.len());
            items.push(val);
        }
        items.push("Add effect".to_string());

        if self.effects.is_some() {
            items.push("Delete effect".to_string());
        }
        items.push("Cancel".to_string());

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Effects menu")
            .items(&items)
            .interact()?;

        match (&self.effects, selection) {
            (Some(_), 0) => Ok(Action::Change(Screen::ViewEffects)),
            (Some(_), 1) => Ok(Action::Change(Screen::AddEffect)),
            (Some(_), 2) => Ok(Action::Change(Screen::DeleteEffects)),
            (Some(_), 3) => Ok(Action::Exit),
            (None, 0) => Ok(Action::Change(Screen::AddEffect)),
            (None, 1) => Ok(Action::Exit),
            (_, _) => Err(anyhow::Error::msg("Invalid selection index")),
        }
    }

    fn change_input_or_output(&mut self, changing_input: bool) -> Result<Action> {
        let path = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter path")
            .interact()?;

        if changing_input {
            match Image::open(&path) {
                Ok(val) => self.input = Some(val),
                Err(e) => {
                    if !fs::exists(&path).unwrap_or(false) {
                        error!("File '{}' does not exist", &path)
                    } else {
                        error!("Error opening file '{}': {}", &path, e)
                    }
                }
            };
        } else {
            match fs::exists(&path) {
                Err(e) => error!("Failed to read '{}': {}", &path, e),
                Ok(true) => warn!("File '{}' already exists and will be overwritten", &path),
                Ok(false) => (),
            }
            self.output = Some(path);
        }

        Ok(Action::Exit)
    }

    fn view_effects(&mut self) -> Result<Action> {
        if let Some(val) = &self.effects {
            val.inner
                .iter()
                .enumerate()
                .for_each(|(i, e)| println!("    {}. {}", i + 1, e));
        } else {
            println!("There are no effects");
        }

        Ok(Action::Exit)
    }

    fn delete_effects(&mut self) -> Result<Action> {
        if let Some(effects) = &mut self.effects {
            let items = effects
                .inner
                .iter()
                .enumerate()
                .map(|(i, e)| format!("    {}. {}", i + 1, e));

            let selection = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Delete effects")
                .items(items)
                .interact()?;

            for i in selection {
                effects.inner.remove(i);
            }
        } else {
            println!("There are no effects")
        }
        Ok(Action::Exit)
    }
}
