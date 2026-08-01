use std::fs;

use anyhow::Result;
use dialoguer::{Input, MultiSelect, Select, theme::ColorfulTheme};
use lib::{Effect, Effects, Image};
use log::warn;

use crate::{action::Action, screen::Screen};

pub struct Manual {
    input: Option<Image>,
    output: Option<String>,
    // TODO: make this non-optional
    effects: Effects,
    screen: Screen,
}

impl Manual {
    pub fn new(input: Option<Image>, output: Option<String>, effects: Option<Effects>) -> Self {
        Self {
            input,
            output,
            effects: effects.unwrap_or_default(),
            screen: Screen::Main,
        }
    }

    pub fn run(mut self) -> Result<()> {
        // TODO: optional interactions
        loop {
            let action = match self.screen {
                Screen::Main => self.render_main(),
                Screen::Image => self.render_image(),
                Screen::ChangeInput => self.change_input(),
                Screen::ChangeOutput => self.change_output(),
                Screen::Effects => self.render_effects(),
                Screen::ViewEffects => self.view_effects(),
                Screen::AddEffect => self.add_effect(),
                Screen::DeleteEffects => self.delete_effects(),
                Screen::LoadEffects => self.load_effects(),
                Screen::SaveEffects => self.save_effects(),
                Screen::Apply => self.apply(),
            }?;

            match (action, self.screen) {
                // if change screen, just change it
                (Action::Change(screen), _) => self.screen = screen,
                // exit from main = quit program
                (Action::Exit, Screen::Main) => break,
                // exit from image submenu
                (Action::Exit, Screen::Image) => self.screen = Screen::Main,
                (Action::Exit, Screen::ChangeInput) => self.screen = Screen::Image,
                (Action::Exit, Screen::ChangeOutput) => self.screen = Screen::Image,
                // exit from effects submenu
                (Action::Exit, Screen::Effects) => self.screen = Screen::Main,
                (Action::Exit, Screen::ViewEffects) => self.screen = Screen::Effects,
                (Action::Exit, Screen::AddEffect) => self.screen = Screen::Effects,
                (Action::Exit, Screen::DeleteEffects) => self.screen = Screen::Effects,
                (Action::Exit, Screen::LoadEffects) => self.screen = Screen::Effects,
                (Action::Exit, Screen::SaveEffects) => self.screen = Screen::Effects,
                // exit from apply submenu
                (Action::Exit, Screen::Apply) => self.screen = Screen::Main,
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
        let effects = format!("Effects ({} defined)", self.effects.len());
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

        if !self.effects.is_empty() {
            let val = format!("View effects ({} defined)", self.effects.len());
            items.push(val);
        }
        items.push("Add effect".to_string());

        if !self.effects.is_empty() {
            items.push("Delete effects".to_string());
        }
        items.push("Load from file".to_string());
        if !self.effects.is_empty() {
            items.push("Save to file".to_string());
        }
        items.push("Cancel".to_string());

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Effects menu")
            .items(&items)
            .interact()?;

        match (!self.effects.is_empty(), selection) {
            (true, 0) => Ok(Action::Change(Screen::ViewEffects)),
            (true, 1) => Ok(Action::Change(Screen::AddEffect)),
            (true, 2) => Ok(Action::Change(Screen::DeleteEffects)),
            (true, 3) => Ok(Action::Change(Screen::LoadEffects)),
            (true, 4) => Ok(Action::Change(Screen::SaveEffects)),
            (true, 5) => Ok(Action::Exit),
            (false, 0) => Ok(Action::Change(Screen::AddEffect)),
            (false, 1) => Ok(Action::Change(Screen::LoadEffects)),
            (false, 2) => Ok(Action::Exit),
            (_, _) => Err(anyhow::Error::msg("Invalid selection index")),
        }
    }

    fn change_input(&mut self) -> Result<Action> {
        let path = Self::file_path_input(false, true)?;

        match Image::open(path) {
            Ok(val) => self.input = Some(val),
            Err(e) => {
                warn!("Error opening image file: {}", e);
                self.input = None;
            }
        }

        Ok(Action::Exit)
    }

    fn change_output(&mut self) -> Result<Action> {
        let path = Self::file_path_input(true, false)?;

        self.output = Some(path);

        Ok(Action::Exit)
    }

    fn view_effects(&mut self) -> Result<Action> {
        if !self.effects.is_empty() {
            self.effects
                .inner
                .iter()
                .enumerate()
                .for_each(|(i, e)| println!("    {}. {}", i + 1, e));
        } else {
            println!("There are no effects");
        }

        Ok(Action::Exit)
    }

    fn delete_effects(&mut self) -> Result<Action> {
        if !self.effects.is_empty() {
            let items = &mut self
                .effects
                .inner
                .iter()
                .enumerate()
                .map(|(i, e)| format!("    {}. {}", i + 1, e));

            let mut selection = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Delete effects")
                .items(items)
                .interact()?;

            // sort from highest to lowest, so the index does not change mid-deleting
            selection.sort_unstable_by(|a, b| b.cmp(a));
            for i in selection {
                self.effects.inner.remove(i);
            }
        } else {
            println!("There are no effects")
        }
        Ok(Action::Exit)
    }

    fn add_effect(&mut self) -> Result<Action> {
        let items = [
            "Brightness",
            "Wrap Brightness",
            "Invert",
            "ReverseBits",
            "Min",
            "Max",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Add Effect")
            .items(items)
            .interact()?;

        let effects = &mut self.effects;

        match selection {
            0 | 1 => {
                let value = Self::number_input(-255, 255)?;

                if selection == 0 {
                    effects.inner.push(Effect::Brightness(value as i16));
                } else {
                    effects.inner.push(Effect::WrapBrightness(value as i16));
                }
            }
            2 => {
                effects.inner.push(Effect::Invert);
            }
            3 => {
                effects.inner.push(Effect::ReverseBits);
            }
            4 | 5 => {
                let value = Self::number_input(0, 255)?;

                if selection == 4 {
                    effects.inner.push(Effect::Min(value as u8));
                } else {
                    effects.inner.push(Effect::Max(value as u8));
                }
            }
            _ => return Err(anyhow::Error::msg("Not implemented")),
        };

        Ok(Action::Exit)
    }

    fn load_effects(&mut self) -> Result<Action> {
        let path = Self::file_path_input(false, true)?;

        match Effects::load(path) {
            Ok(val) => self.effects = val,
            Err(e) => {
                warn!("Failed opening effects file: {}", e);
                self.effects.inner = Vec::default()
            }
        }

        Ok(Action::Exit)
    }

    fn save_effects(&mut self) -> Result<Action> {
        if !self.effects.is_empty() {
            let path = Self::file_path_input(true, false)?;

            if let Err(e) = self.effects.save(path) {
                warn!("Error saving effects to file: {}", e);
            }
        } else {
            warn!("No effects")
        }

        Ok(Action::Exit)
    }

    fn number_input(min: i32, max: i32) -> Result<i32> {
        let prompt = format!("Enter value ({} to {})", min, max);

        let value = Input::<i32>::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .validate_with(|input: &i32| -> Result<(), &str> {
                if *input > max {
                    Err("Value too big")
                } else if *input < min {
                    Err("Value too small")
                } else {
                    Ok(())
                }
            })
            .interact()?;

        Ok(value)
    }

    fn apply(&mut self) -> Result<Action> {
        if let Some(i) = &self.input
            && let Some(o) = &self.output
            && !&self.effects.inner.is_empty()
        {
            let mut image = i.clone();
            image.effects(&self.effects);
            if let Err(err) = image.save(o) {
                warn!("Failed to save image: {}", err)
            }
        } else {
            warn!("You need to specify input, output and at least one effect")
        }

        Ok(Action::Exit)
    }

    fn file_path_input(warn_existing: bool, warn_nonexisting: bool) -> Result<String> {
        let path = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter path")
            .interact()?;

        if warn_existing {
            match fs::exists(&path) {
                Ok(true) => warn!("File already exists and will be overwritten"),
                Ok(false) => (),
                Err(_) => warn!("If this file exists, it will be overwritten"),
            }
        }

        if warn_nonexisting {
            match fs::exists(&path) {
                Ok(true) => (),
                Ok(false) => warn!("File does not exist"),
                Err(_) => warn!("File might not exist"),
            }
        }

        Ok(path)
    }
}
