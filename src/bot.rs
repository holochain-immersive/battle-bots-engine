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
        Bot {
            energy: BOTS_STARTING_ENERGY,
            color,
            shield_direction: Direction::Up,
            chainsaw_direction: Direction::Left,
            tiredness: 0,
            shield_resistance: STARTING_SHIELD_RESISTANCE,
        }
    }

    pub fn gain_energy(&mut self, energy_gain: usize) {
        self.energy += energy_gain;

        if self.energy > BOTS_STARTING_ENERGY {
            self.energy = BOTS_STARTING_ENERGY
        }
    }

    pub fn is_shield_destroyed(&self) -> bool {
        self.shield_resistance == 0
    }

    pub fn move_step(&mut self) {
        self.tiredness += 1;

        if self.tiredness >= TIREDNESS_TO_LOSE_ENERGY {
            self.tiredness = 0;
            self.energy -= 1;
        }
    }

    pub fn receive_attack(&mut self, from_direction: Direction) {
        if self.shield_direction.opposite().eq(&from_direction) && self.shield_resistance > 0 {
            if self.shield_resistance > ATTACK_DAMAGE {
                self.shield_resistance -= ATTACK_DAMAGE;
            } else {
                self.shield_resistance = 0;
            }
        } else {
            if self.energy > ATTACK_DAMAGE {
                self.energy -= ATTACK_DAMAGE;
            } else {
                self.energy = 0;
            }
        }
    }
}
