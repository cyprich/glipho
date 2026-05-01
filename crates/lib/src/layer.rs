use std::fmt::Display;

#[derive(Debug)]
pub enum Layer {
    Brightness(i16),
    WrapBrightness(i16),
    Invert,
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Layer::Brightness(val) => format!("Saturating Add ({})", val),
            Layer::WrapBrightness(val) => format!("Wrapping Add ({})", val),
            Layer::Invert => "Invert".into(),
        };

        write!(f, "{}", result)
    }
}
