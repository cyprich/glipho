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
        let result = match self {
            Layer::Brightness(val) => format!("Brightness {}", val),
            Layer::WrapBrightness(val) => format!("Wrapping Brightness {}", val),
            Layer::Invert => "Invert".into(),
            Layer::ReverseBits => "Reverse Bits".into(),
            Layer::Min(val) => format!("Min {}", val),
            Layer::Max(val) => format!("Max {}", val),
        };

        write!(f, "{}", result)
    }
}
