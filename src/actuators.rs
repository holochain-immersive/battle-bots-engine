use super::{
    state::{from_matrix, state_to_matrix, GameCell, GameState},
    direction::{Direction, Rotation},
};

pub struct Actuators {
    pub rotate_shield: Option<Rotation>,
    pub rotate_chainsaw: Option<Rotation>,
    pub move_bot: Option<Direction>,
}

impl Actuators {
    pub fn execute(
        &self,
        bot_pos_x: usize,
        bot_pos_y: usize,
        mut game_state: GameState,
    ) -> GameState {
        if let Some(rotation) = self.rotate_chainsaw {
            game_state = rotate_chainsaw(rotation, bot_pos_x, bot_pos_y, game_state);
        }
        if let Some(rotation) = self.rotate_shield {
            game_state = rotate_shield(rotation, bot_pos_x, bot_pos_y, game_state);
        }

        if let Some(direction) = self.move_bot {
            game_state = move_bot(direction, bot_pos_x, bot_pos_y, game_state);
        }

        game_state
    }
}

fn move_bot(
    direction: Direction,
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: GameState,
) -> GameState {
    let (final_position_x, final_position_y) = direction.compute_position(bot_pos_x, bot_pos_y);
    let mut map = state_to_matrix(game_state);

    if let GameCell::Bot(mut bot) = map[bot_pos_x][bot_pos_y] {
        if let GameCell::Resource(r) = map[final_position_x][final_position_y] {
            bot.gain_energy(r.energy_gain);

            bot.move_step();
            map[final_position_x][final_position_y] = GameCell::Bot(bot);
            map[bot_pos_x][bot_pos_y] = GameCell::Empty;
        } else if let GameCell::Empty = map[final_position_x][final_position_y] {
            bot.move_step();
            map[final_position_x][final_position_y] = GameCell::Bot(bot);
            map[bot_pos_x][bot_pos_y] = GameCell::Empty;
        }
    }
    from_matrix(map)
}

fn rotate_shield(
    rotation: Rotation,
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: GameState,
) -> GameState {
    let mut map = state_to_matrix(game_state);

    if let GameCell::Bot(mut bot) = map[bot_pos_x][bot_pos_y] {
        bot.shield_direction = bot.shield_direction.rotate(rotation);

        map[bot_pos_x][bot_pos_y] = GameCell::Bot(bot);
    }
    from_matrix(map)
}

fn rotate_chainsaw(
    rotation: Rotation,
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: GameState,
) -> GameState {
    let mut map = state_to_matrix(game_state);

    if let GameCell::Bot(mut bot) = map[bot_pos_x][bot_pos_y] {
        bot.chainsaw_direction = bot.chainsaw_direction.rotate(rotation);

        let (attacking_position_x, attacking_position_y) = bot
            .chainsaw_direction
            .compute_position(bot_pos_x, bot_pos_y);

        if let GameCell::Bot(mut attacked_bot) = map[attacking_position_x][attacking_position_y] {
            attacked_bot.receive_attack(bot.chainsaw_direction);
            map[attacking_position_x][attacking_position_y] = GameCell::Bot(attacked_bot);
        }

        map[bot_pos_x][bot_pos_y] = GameCell::Bot(bot);
    }
    from_matrix(map)
}
