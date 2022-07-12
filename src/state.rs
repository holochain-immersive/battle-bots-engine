use super::{bot::Bot, constants::*, resource::Resource};

#[derive(Clone, Copy)]
pub enum GameCell {
    Empty,
    Bot(Bot),
    Resource(Resource),
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub map_width: usize,
    pub map_height: usize,
    pub bots: Vec<(Position, Bot)>,
    pub resources: Vec<(Position, Resource)>,
}

pub(crate) fn state_to_matrix(state: GameState) -> [[GameCell; MAP_HEIGHT]; MAP_WIDTH] {
    let mut map = [[GameCell::Empty; MAP_HEIGHT]; MAP_WIDTH];

    for (pos, bot) in state.bots {
        map[pos.x][pos.y] = GameCell::Bot(bot);
    }
    for (pos, resource) in state.resources {
        map[pos.x][pos.y] = GameCell::Resource(resource);
    }

    map
}

pub(crate) fn from_matrix(matrix: [[GameCell; MAP_HEIGHT]; MAP_WIDTH]) -> GameState {
    let mut state = GameState {
        bots: vec![],
        resources: vec![],
        map_width: MAP_WIDTH,
        map_height: MAP_HEIGHT,
    };

    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            match matrix[x][y] {
                GameCell::Bot(bot) => state.bots.push((Position { x, y }, bot)),
                GameCell::Resource(resource) => state.resources.push((Position { x, y }, resource)),
                _ => {}
            }
        }
    }

    state
}
