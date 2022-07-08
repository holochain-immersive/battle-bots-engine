use crate::*;

pub fn red(
    game_state: &GameState,
    bot_position: Position,
    bot_in_position: fn(&GameState, &Position) -> Option<Bot>,
    valid_adjacent_positions: fn(&GameState, &Position) -> Vec<Position>,
    adjacent_positions_to_direction: fn(&Position, &Position) -> Result<Direction, String>,
    adjacent_bot: fn(&GameState, &Position) -> Option<Direction>,
    get_closest_enemy: fn(&GameState, &Position) -> Option<Position>,
) -> Actuators {
    // Returns whether the position (x, y) is inside the map bounds
    // eg. is_position_inside_map_bounds(0, 1, 2, 2) == true, is_position_inside_map_bounds(2, 1, 2, 2) == false
    let is_position_inside_map_bounds =
        |x: usize, y: usize, map_width: usize, map_height: usize| x < map_width && y < map_height;

    // Returns the shortest way to rotate the "from" direction to get the "to" direction
    // Assumes that from and to are not equal
    // eg. shortest_rotation(Direction::Up, Direction::Right) == Rotation::Clockwise
    let shortest_rotation = |from: &Direction, to: &Direction| match (from, to) {
        (Direction::Down, Direction::Left)
        | (Direction::Left, Direction::Up)
        | (Direction::Up, Direction::Right)
        | (Direction::Right, Direction::Down) => Rotation::Clockwise,
        _ => Rotation::Counterclockwise,
    };

    // Rotate the given direction with the given rotation
    // eg. rotate_direction(Direction::Up, Rotation::Clockwise) == Direction::Right
    let rotate_direction = |direction: &Direction, rotation: &Rotation| match rotation {
        Rotation::Clockwise => match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        },
        Rotation::Counterclockwise => match direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        },
    };

    // Control which way the shield should rotate
    // If returns None, the shield won't rotate at all
    let shield_rotation = |game_state: &GameState, bot_position: &Position| {
        let maybe_bot = bot_in_position(game_state, &bot_position);

        if let Some(bot) = maybe_bot {
            if let Some(adjacent_bot_direction) = adjacent_bot(game_state, bot_position) {
                if bot.shield_direction != adjacent_bot_direction {
                    let rotation =
                        shortest_rotation(&bot.shield_direction, &adjacent_bot_direction);

                    return Some(rotation);
                }
            }
        }

        None
    };

    // Controls which way the chainsaw should rotate
    // If returns None, the chainsaw won't rotate at all
    let chainsaw_rotation = |game_state: &GameState, bot_position: &Position| {
        let maybe_bot = bot_in_position(game_state, &bot_position);

        if let Some(bot) = maybe_bot {
            if let Some(adjacent_bot_direction) = adjacent_bot(game_state, bot_position) {
                if adjacent_bot_direction
                    == rotate_direction(
                        &rotate_direction(&bot.chainsaw_direction, &Rotation::Counterclockwise),
                        &Rotation::Clockwise,
                    )
                {
                    return Some(Rotation::Clockwise);
                }

                let rotation = shortest_rotation(&bot.chainsaw_direction, &adjacent_bot_direction);

                return Some(rotation);
            }
        }

        None
    };

    let find_shortest_path =
        |game_state: &GameState, from: &Position, to: &Position| -> Result<Vec<Position>, String> {
            // BFS

            let mut visited = vec![vec![false; game_state.map_height]; game_state.map_width];
            let mut queue: Vec<(Position, Vec<Position>)> = vec![];

            visited[from.x][from.y] = true;
            queue.push((from.clone(), vec![]));

            while !queue.is_empty() {
                let (current_pos, path) = queue.remove(0);

                if current_pos.x == to.x && current_pos.y == to.y {
                    let mut new_path = path.clone();

                    new_path.push(Position {
                        x: current_pos.x,
                        y: current_pos.y,
                    });
                    new_path.remove(0);

                    return Ok(new_path);
                }

                let adjacents = valid_adjacent_positions(game_state, &current_pos);

                for adjacent_pos in adjacents {
                    if is_position_inside_map_bounds(
                        adjacent_pos.x,
                        adjacent_pos.y,
                        game_state.map_width,
                        game_state.map_height,
                    ) && !visited[adjacent_pos.x][adjacent_pos.y]
                    {
                        visited[adjacent_pos.x][adjacent_pos.y] = true;

                        let mut new_path = path.clone();

                        new_path.push(Position {
                            x: current_pos.x,
                            y: current_pos.y,
                        });

                        queue.push((adjacent_pos, new_path));
                    }
                }
            }

            Err("There is no available path".into())
        };

    // Returns the direction of the next move in the path to go from the "from" position to the "to" position
    let next_move_in_path = |game_state: &GameState, from: &Position, to: &Position| {
        let moves = find_shortest_path(game_state, from, to)?;

        let first_move_position = moves
            .first()
            .ok_or(String::from("No moves to the chosen path"))?
            .clone();

        adjacent_positions_to_direction(from, &first_move_position)
    };

    let next_move_towards_enemy = |game_state: &GameState, bot_position: &Position| {
        if let Some(closest_enemy_position) = get_closest_enemy(game_state, bot_position) {
            if let Ok(next_move) =
                next_move_in_path(game_state, bot_position, &closest_enemy_position)
            {
                return Some(next_move);
            }
        }

        None
    };

    let shield_rotation = shield_rotation(game_state, &bot_position);

    let chainsaw_rotation = chainsaw_rotation(game_state, &bot_position);

    let move_bot = next_move_towards_enemy(game_state, &bot_position);

    Actuators {
        rotate_chainsaw: chainsaw_rotation,
        rotate_shield: shield_rotation,
        move_bot,
    }
}
