#[derive(Debug)]
pub enum Layer {
    AddSaturating(u8),
    AddWrapping(u8),
    Invert,
}
