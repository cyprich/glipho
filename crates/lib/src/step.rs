use anyhow::{Context, Result};
use std::{fmt::Display, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Step {
    Layer(crate::Layer),
    Save(String),
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Step::Layer(val) => format!("Layer {}", val),
            Step::Save(val) => format!("Save  {}", val),
        };

        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Steps {
    pub steps: Vec<Step>,
}

impl Steps {
    pub fn steps_string(&self) -> Vec<String> {
        self.steps.iter().map(|s| s.to_string()).collect()
    }

    pub fn push(&mut self, step: Step) {
        self.steps.push(step);
    }

    pub fn remove(&mut self, index: usize) {
        self.steps.remove(index);
    }

    pub fn to_toml_string(&self) -> Result<String> {
        let result = toml::to_string(self).context("Failed to serialize Steps")?;
        Ok(result)
    }

    pub fn from_toml_string(string: &str) -> Result<Self> {
        let result: Self = toml::from_str(string)?;
        Ok(result)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let text = fs::read_to_string(path).context(format!("Failed to read file {}", path))?;
        Self::from_toml_string(&text)
    }

    pub fn is_saved(&self) -> bool {
        if self.steps.is_empty() {
            true
        } else {
            matches!(self.steps.last().unwrap(), Step::Save(_))
        }
    }
}
