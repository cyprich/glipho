pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

// impl Color {
//     pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
//         Self { r, g, b, a }
//     }
// }

impl Into<slint::Color> for Color {
    fn into(self) -> slint::Color {
        let c = slint::Color::from_rgb_u8(self.r, self.g, self.b);
        c.with_alpha(self.a)
    }
}

pub const INDICATOR_BLUE: Color = Color {
    r: 43,
    g: 127,
    b: 255,
    a: 1.0,
};

pub const INDICATOR_VIOLET: Color = Color {
    r: 142,
    g: 81,
    b: 255,
    a: 1.0,
};
