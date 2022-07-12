use ruscii::terminal::Color;

use super::{
    actuators::Actuators,
    state::{GameState, Position},
    direction::Direction,
};
use super::constants::*;

pub type BotStrategy = fn(&GameState, Position) -> Actuators;

pub struct ColorConfig {
    pub color: Color,
    pub number_of_bots: usize,
    pub strategy: BotStrategy,
}

pub struct Bot {
    pub energy: usize,
    pub color: Color,
    pub chainsaw_direction: Direction,
    pub shield_direction: Direction,
    pub tiredness: usize,
    pub shield_resistance: usize,
}
