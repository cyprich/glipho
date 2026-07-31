use crate::screen::Screen;

pub enum Action {
    Change(Screen),
    Exit,
}
