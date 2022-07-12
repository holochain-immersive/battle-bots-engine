mod actuators;
mod bot;
mod direction;
mod resource;
mod state;

mod battle;
mod broken_bots;
mod constants;

pub use actuators::Actuators;
pub use battle::Battle;
pub use bot::{Bot, BotStrategy, ColorConfig};
pub use broken_bots::{blue::blue, grey::grey, red::red, yellow::yellow};
pub use direction::{Direction, Rotation};
pub use resource::Resource;
pub use ruscii::terminal::Color;
pub use state::{GameState, Position};
