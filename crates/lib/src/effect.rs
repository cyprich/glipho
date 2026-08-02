use std::{fmt::Display, fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Effect {
    Brightness(i16),
    WrapBrightness(i16),
    Invert,
    ReverseBits,
    Min(u8),
    Max(u8),
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Invert | Effect::ReverseBits => write!(f, "{}", self.to_type()),
            _ => write!(f, "{} {}", self.to_type(), self.to_value()),
        }
    }
}

impl Effect {
    pub fn try_from_name(name: &str, value: i32) -> Option<Self> {
        let name = name.to_lowercase().replace(' ', "");
        let result = match name.as_str() {
            "brightness" => Self::Brightness(value as i16),
            "wrappedbrightness" => Self::WrapBrightness(value as i16),
            "invert" => Self::Invert,
            "reversebits" => Self::ReverseBits,
            "min" => Self::Min(value as u8),
            "max" => Self::Max(value as u8),
            _ => return None,
        };

        Some(result)
    }

    pub fn to_type(&self) -> String {
        match self {
            Effect::Brightness(_) => "Brightness",
            Effect::WrapBrightness(_) => "Wrapping Brightness",
            Effect::Invert => "Invert",
            Effect::ReverseBits => "Reverse Bits",
            Effect::Min(_) => "Min",
            Effect::Max(_) => "Max",
        }
        .into()
    }

    pub fn to_value(&self) -> String {
        match self {
            Effect::Brightness(val) | Effect::WrapBrightness(val) => val.to_string(),
            Effect::Min(val) | Effect::Max(val) => val.to_string(),
            _ => "".into(),
        }
    }

    pub fn change_value(&mut self, new_value: i32) -> bool {
        match self {
            Effect::Brightness(val) | Effect::WrapBrightness(val) => {
                *val = new_value as i16;
                true
            }
            Effect::Min(val) | Effect::Max(val) => {
                *val = new_value as u8;
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)] // so the result is not wrapped in `inner`
pub struct Effects {
    pub inner: Vec<Effect>,
}

impl Effects {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let text = fs::read_to_string(path)
            .context(format!("Failed to read file '{}'", path.to_string_lossy()))?;

        let result: Self =
            serde_json::from_str(&text).context("Failed to deserialize Effects from file")?;

        Ok(result)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let text = serde_json::to_string(self).context("Couldn't serialize Effect to file")?;

        fs::write(path, text).context("Failed to save to file")?;

        Ok(())
    }

    pub fn change_value(&mut self, id: usize, new_value: i32) -> bool {
        let effect = self.inner.get_mut(id);

        if let Some(effect) = effect {
            effect.change_value(new_value)
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, value: Effect) {
        self.inner.push(value)
    }

    pub fn remove(&mut self, index: usize) -> Effect {
        self.inner.remove(index)
    }
}
