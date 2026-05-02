use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Layer {
    Brightness(i16),
    WrapBrightness(i16),
    Invert,
    ReverseBits,
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Layer::Brightness(val) => format!("Brightness {}", val),
            Layer::WrapBrightness(val) => format!("Wrapping Brightness {}", val),
            Layer::Invert => "Invert".into(),
            Layer::ReverseBits => "Reverse Bits".into(),
        };

        write!(f, "{}", result)
    }
}
