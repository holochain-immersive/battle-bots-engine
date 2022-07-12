use super::constants::{MAP_HEIGHT, MAP_WIDTH};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub enum Rotation {
    Clockwise,
    Counterclockwise,
}
