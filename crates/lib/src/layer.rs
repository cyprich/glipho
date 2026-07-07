use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Layer {
    Brightness(i16),
    WrapBrightness(i16),
    Invert,
    ReverseBits,
    Min(u8),
    Max(u8),
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layer::Invert | Layer::ReverseBits => write!(f, "{}", self.to_type()),
            _ => write!(f, "{} {}", self.to_type(), self.to_value()),
        }
    }
}

impl Layer {
    pub fn to_type(&self) -> String {
        match self {
            Layer::Brightness(_) => "Brightness",
            Layer::WrapBrightness(_) => "Wrapping Brightness",
            Layer::Invert => "Invert",
            Layer::ReverseBits => "Reverse Bits",
            Layer::Min(_) => "Min",
            Layer::Max(_) => "Max",
        }
        .into()
    }

    pub fn to_value(&self) -> String {
        match self {
            Layer::Brightness(val) | Layer::WrapBrightness(val) => val.to_string(),
            Layer::Min(val) | Layer::Max(val) => val.to_string(),
            _ => "".into(),
        }
    }
}
