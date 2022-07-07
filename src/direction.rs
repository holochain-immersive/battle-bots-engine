use super::constants::{MAP_HEIGHT, MAP_WIDTH};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
}

impl Direction {
    pub fn compute_position(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up if y + 1 < MAP_HEIGHT => (x, y + 1),
            Direction::Down if (y as isize) - 1 >= 0 => (x, y - 1),
            Direction::Right if x + 1 < MAP_WIDTH => (x + 1, y),
            Direction::Left if (x as isize) - 1 >= 0 => (x - 1, y),
            _ => (x, y),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    pub fn rotate(&self, rotation: Rotation) -> Direction {
        match rotation {
            Rotation::Clockwise => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            },
            Rotation::Counterclockwise => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
            },
        }
    }
}
