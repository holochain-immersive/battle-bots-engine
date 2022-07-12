use super::{
    direction::{Direction, Rotation},
};

pub struct Actuators {
    pub rotate_shield: Option<Rotation>,
    pub rotate_chainsaw: Option<Rotation>,
    pub move_bot: Option<Direction>,
}
