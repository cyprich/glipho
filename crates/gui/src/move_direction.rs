use crate::slint_generatedApp;

pub fn move_direction_from_slint(
    direction: &slint_generatedApp::MoveDirection,
) -> lib::MoveDirection {
    match direction {
        crate::MoveDirection::Up => lib::MoveDirection::Up,
        crate::MoveDirection::Down => lib::MoveDirection::Down,
    }
}
