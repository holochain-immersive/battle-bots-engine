use super::{bot::Bot, constants::*, resource::Resource};

pub enum GameCell {
    Empty,
    Bot(Bot),
    Resource(Resource),
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct GameState {
    pub map_width: usize,
    pub map_height: usize,
    pub bots: Vec<(Position, Bot)>,
    pub resources: Vec<(Position, Resource)>,
}
