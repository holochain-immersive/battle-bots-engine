use ruscii::terminal::Color;

use super::constants::*;
use super::{
    actuators::Actuators,
    direction::Direction,
    state::{GameState, Position},
};

pub type BotStrategy = fn(&GameState, Position) -> Actuators;

pub struct ColorConfig {
    pub color: Color,
    pub number_of_bots: usize,
    pub strategy: BotStrategy,
}

#[derive(Clone, Copy)]
pub struct Bot {
    pub energy: usize,
    pub color: Color,
    pub chainsaw_direction: Direction,
    pub shield_direction: Direction,
    pub tiredness: usize,
    pub shield_resistance: usize,
}

impl Bot {
    pub fn new(color: Color) -> Bot {
        unimplemented!()
    }

    pub fn is_shield_destroyed(&self) -> bool {
        false
    }
}
