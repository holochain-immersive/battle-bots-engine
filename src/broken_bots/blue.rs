use crate::*;

// Return the sum of a and b
fn sum(a: usize, b: usize) -> usize {
    a + b
}

// Returns whether the two given position are the same position
fn are_positions_equal(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    x1 == x2 && y1 == y2
}

// Returns whether the position (x, y) is inside the map bounds
// eg. is_position_inside_map_bounds(0, 1, 2, 2) == true, is_position_inside_map_bounds(2, 1, 2, 2) == false
fn is_position_inside_map_bounds(x: usize, y: usize, map_width: usize, map_height: usize) -> bool {
    x < map_width && y < map_height
}

// If n is a positive integer, returns n
// if n is a negative integer, returns -n
fn absolute(n: isize) -> usize {
    if n < 0 {
        return -n as usize;
    } else {
        return n as usize;
    }
}

// Returns the distance from one position to another, counting the number of non-diagonal steps between them
// eg. distance(0, 0, 1, 1) == 2
fn distance(
    from_pos_x: usize,
    from_pos_y: usize,
    to_pos_x: usize,
    to_pos_y: usize,
) -> usize {
    let x_distance = absolute(to_pos_x as isize - from_pos_x as isize);
    let y_distance = absolute(to_pos_y as isize - from_pos_y as isize);

    x_distance + y_distance
}

// Returns the position that's adjacent to the left of the given one, in the form (x, y)
// eg. adjacent_position_to_the_left(4, 5) == (3, 5)
fn adjacent_position_to_the_left(x: usize, y: usize) -> (usize, usize) {
    (x - 1, y)
}

// Returns the position that's adjacent to the given one in the given direction, in the form (x, y)
// eg. adjacent_position_in_direction(4, 5, Direction::Down) == (4, 6)
fn adjacent_position_in_direction(
    x: usize,
    y: usize,
    direction: Direction,
) -> (usize, usize) {
    match direction {
        Direction::Down => (x, sum(y, 1)),
        Direction::Up => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

// Returns whether there is a bot in the given position
fn is_bot(
    game_state: &GameState,
    position: &Position,
) -> bool {
    bot_in_position(game_state, position).is_some()
}

// Returns the shortest way to rotate the "from" direction to get the "to" direction
// Assumes that from and to are not equal
// eg. shortest_rotation(Direction::Up, Direction::Right) == Rotation::Clockwise
fn shortest_rotation(from: &Direction, to: &Direction) -> Rotation {
    match (from, to) {
        (Direction::Down, Direction::Left)
        | (Direction::Left, Direction::Up)
        | (Direction::Up, Direction::Right)
        | (Direction::Right, Direction::Down) => Rotation::Clockwise,
        _ => Rotation::Counterclockwise,
    }
}

// Rotate the given direction with the given rotation
// eg. rotate_direction(Direction::Up, Rotation::Clockwise) == Direction::Right
fn rotate_direction(direction: &Direction, rotation: &Rotation) -> Direction {
    match rotation {
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
    }
}

/** Lesson 5 */

// Returns a bot if there is one in the given position
fn bot_in_position(game_state: &GameState, position: &Position) -> Option<Bot> {
    game_state
        .bots
        .iter()
        .find(|b| b.0.x == position.x && b.0.y == position.y)
        .map(|(_, b)| b.clone())
}

// Filter out the positions that are not in the bounds of the map
// eg. filter_valid_positions(vec![Position { x: 1, y: 1}, Position { x: 2, y: 1}], 2, 2) == vec![Position { x: 1, y: 1}]
fn filter_valid_positions(
    positions: Vec<Position>,
    map_width: usize,
    map_height: usize,
) -> Vec<Position> {
    positions
        .into_iter()
        .filter(|pos| is_position_inside_map_bounds(pos.x, pos.y, map_width, map_height))
        .collect()
}

// Return a vector of the adjacent positions to the given one, in the form of (x, y) tuples
// Careful! Don't return invalid positions (negative coordinates, or coordinates that exceed the map size)
fn valid_adjacent_positions(
    game_state: &GameState,
    position: &Position,
) -> Vec<Position> {
    let mut positions = vec![];

    if position.x > 0 {
        positions.push(adjacent_position_to_the_left(position.x, position.y));
    }
    positions.push(adjacent_position_in_direction(
        position.x,
        position.y,
        Direction::Right,
    ));

    if position.y > 0 {
        positions.push(adjacent_position_in_direction(
            position.x,
            position.y,
            Direction::Up,
        ));
    }
    positions.push(adjacent_position_in_direction(
        position.x,
        position.y,
        Direction::Down,
    ));

    let positions = positions
        .into_iter()
        .map(|(x, y)| Position { x, y })
        .collect();

    filter_valid_positions(positions, game_state.map_width, game_state.map_height)
}

// Returns the direction that the to position is relative to the from position
// eg: adjacent_positions_to_direction(Position { x: 0, y: 0 }, Position { x: 1, y: 0 }) == Direction::Right
fn adjacent_positions_to_direction(from: &Position, to: &Position) -> Result<Direction, String> {
    if from.x + 1 == to.x && from.y == to.y {
        return Ok(Direction::Right);
    } else if from.x == to.x + 1 && from.y == to.y {
        return Ok(Direction::Left);
    } else if from.x == to.x && from.y + 1 == to.y {
        return Ok(Direction::Up);
    } else if from.x == to.x && from.y == to.y + 1 {
        return Ok(Direction::Down);
    }

    Err(String::from("Positions are not adjacent"))
}

// Returns whether there is an adjacent bot, and its position if there is one
fn adjacent_bot(
    game_state: &GameState,
    bot_position: &Position,
) -> Option<Direction> {
    let adjacent = valid_adjacent_positions(game_state, bot_position);

    let maybe_adjacent_bot = adjacent
        .into_iter()
        .filter(|position| {
            is_position_inside_map_bounds(
                position.x,
                position.y,
                game_state.map_width,
                game_state.map_height,
            )
        })
        .find(|pos| is_bot(game_state, pos));

    if let Some(adjacent_bot) = maybe_adjacent_bot {
        if let Ok(adjacent_bot_direction) =
            adjacent_positions_to_direction(bot_position, &adjacent_bot)
        {
            return Some(adjacent_bot_direction);
        }
    }
    None
}

// Control which way the shield should rotate
// If returns None, the shield won't rotate at all
fn shield_rotation(game_state: &GameState, bot_position: &Position) -> Option<Rotation> {
    let maybe_bot = bot_in_position(game_state, &bot_position);

    if let Some(bot) = maybe_bot {
        if let Some(adjacent_bot_direction) = adjacent_bot(game_state, bot_position) {
            if bot.shield_direction != adjacent_bot_direction {
                let rotation = shortest_rotation(&bot.shield_direction, &adjacent_bot_direction);

                return Some(rotation);
            }
        }
    }

    None
}

// Controls which way the chainsaw should rotate
// If returns None, the chainsaw won't rotate at all
fn chainsaw_rotation(game_state: &GameState, bot_position: &Position) -> Option<Rotation> {
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
}

// Returns the position of the closest enemy
fn get_closest_enemy(
    game_state: &GameState,
    bot_position: &Position,
) -> Option<Position> {
    let mut closest_enemy: Option<Position> = None;

    for (position, _bot) in game_state.bots.iter() {
        if !are_positions_equal(bot_position.x, bot_position.y, position.x, position.y) {
            match closest_enemy {
                Some(Position {
                    x: closest_x,
                    y: closest_y,
                }) if distance(closest_x, closest_y, bot_position.x, bot_position.y)
                    < distance(position.x, position.y, bot_position.x, bot_position.y) => {}
                _ => {
                    closest_enemy = Some(Position {
                        x: position.x,
                        y: position.y,
                    })
                }
            };
        }
    }

    closest_enemy
}

// Returns the direction of the next move in the path to go from the "from" position to the "to" position
fn next_move_in_path(
    game_state: &GameState,
    from: &Position,
    to: &Position,
) -> Result<Direction, String> {
    let moves = find_shortest_path(game_state, from, to)?;

    let first_move_position = moves
        .first()
        .ok_or(String::from("No moves to the chosen path"))?
        .clone();

    adjacent_positions_to_direction(from, &first_move_position)
}

fn find_shortest_path(
    game_state: &GameState,
    from: &Position,
    to: &Position,
) -> Result<Vec<Position>, String> {
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
}

fn next_move_towards_enemy(game_state: &GameState, bot_position: &Position) -> Option<Direction> {
    if let Some(closest_enemy_position) = get_closest_enemy(game_state, bot_position) {
        if let Ok(next_move) = next_move_in_path(game_state, bot_position, &closest_enemy_position)
        {
            return Some(next_move);
        }
    }

    None
}

/**
 * Rotate the shield towards and adjacent enemy
 * Attack an adjacent enemy with the chainsaw
 * Move towards the closest enemy
 */
pub fn blue(game_state: &GameState, bot_position: Position) -> Actuators {
    let shield_rotation = shield_rotation(game_state, &bot_position);

    let chainsaw_rotation = chainsaw_rotation(game_state, &bot_position);

    let move_bot = next_move_towards_enemy(game_state, &bot_position);

    Actuators {
        rotate_chainsaw: chainsaw_rotation,
        rotate_shield: shield_rotation,
        move_bot,
    }
}
