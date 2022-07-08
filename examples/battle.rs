use battle_bots_engine::*;

fn main() {
    Battle::new(vec![
        ColorConfig {
            color: Color::Blue,
            number_of_bots: 3,
            strategy: decide,
        },
        ColorConfig {
            color: Color::Yellow,
            number_of_bots: 3,
            strategy: decide,
        },
        ColorConfig {
            color: Color::Grey,
            number_of_bots: 3,
            strategy: decide,
        },
        ColorConfig {
            color: Color::Red,
            number_of_bots: 3,
            strategy: decide,
        },
    ])
    .run()
}


/**
* Rotate the shield towards and adjacent enemy
* Attack an adjacent enemy with the chainsaw
* Move towards the closest enemy
*/
pub fn decide(game_state: &GameState, bot_position: Position) -> Actuators {
  let shield_rotation = shield_rotation(game_state, &bot_position);

  let chainsaw_rotation = chainsaw_rotation(game_state, &bot_position);

  let move_bot = should_move_towards_enemy(game_state, &bot_position);

  Actuators {
      rotate_chainsaw: chainsaw_rotation,
      rotate_shield: shield_rotation,
      move_bot,
  }
}

fn shield_rotation(game_state: &GameState, bot_position: &Position) -> Option<Rotation> {
  let maybe_bot = bot_in_position(game_state, &bot_position);

  if let Some(bot) = maybe_bot {
      if let Some(adjacent_bot_direction) = adjacent_bot(game_state, bot_position) {
          let rotation = shortest_rotation(bot.shield_direction, adjacent_bot_direction);

          return rotation;
      }
  }

  None
}

fn chainsaw_rotation(game_state: &GameState, bot_position: &Position) -> Option<Rotation> {
  let maybe_bot = bot_in_position(game_state, &bot_position);

  if let Some(bot) = maybe_bot {
      if let Some(adjacent_bot_direction) = adjacent_bot(game_state, bot_position) {
          if adjacent_bot_direction == bot.chainsaw_direction {
              return Some(Rotation::Clockwise);
          }

          let rotation = shortest_rotation(bot.chainsaw_direction, adjacent_bot_direction);

          return rotation;
      }
  }

  None
}

fn adjacent_bot(game_state: &GameState, bot_position: &Position) -> Option<Direction> {
  let adjacent = valid_adjacent_positions(game_state, bot_position);

  let maybe_adjacent_bot = adjacent.into_iter().find(|pos| is_bot(game_state, pos));

  if let Some(adjacent_bot) = maybe_adjacent_bot {
      if let Ok(adjacent_bot_direction) =
          adjacent_positions_to_direction(bot_position, &adjacent_bot)
      {
          return Some(adjacent_bot_direction);
      }
  }
  None
}

// Return a vector of the adjacent positions to the given one, in the form of (x, y) tuples
// Careful! Don't return invalid positions (negative coordinates, or coordinates that exceed the map size)
pub fn valid_adjacent_positions(game_state: &GameState, position: &Position) -> Vec<Position> {
  let mut positions = vec![];

  if position.x > 0 {
      positions.push(Position {
          x: position.x - 1,
          y: position.y,
      });
  }
  if position.x < game_state.map_width - 1 {
      positions.push(Position {
          x: position.x + 1,
          y: position.y,
      });
  }
  if position.y > 0 {
      positions.push(Position {
          x: position.x,
          y: position.y - 1,
      });
  }
  if position.y < game_state.map_height - 1 {
      positions.push(Position {
          x: position.x,
          y: position.y + 1,
      });
  }

  positions
}

pub fn is_bot(game_state: &GameState, position: &Position) -> bool {
  bot_in_position(game_state, position).is_some()
}

pub fn bot_in_position(game_state: &GameState, position: &Position) -> Option<Bot> {
  game_state
      .bots
      .iter()
      .find(|b| b.0.x == position.x && b.0.y == position.y)
      .map(|(_, b)| b.clone())
}

pub fn adjacent_positions_to_direction(
  from: &Position,
  to: &Position,
) -> Result<Direction, String> {
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

fn shortest_rotation(from: Direction, to: Direction) -> Option<Rotation> {
  match (from, to) {
      _ if from == to => None,
      (Direction::Down, Direction::Left)
      | (Direction::Left, Direction::Up)
      | (Direction::Up, Direction::Right)
      | (Direction::Right, Direction::Down) => Some(Rotation::Clockwise),
      _ => Some(Rotation::Counterclockwise),
  }
}

pub fn absolute(n: isize) -> usize {
  if n < 0 {
      return -n as usize;
  } else {
      return n as usize;
  }
}

pub fn should_move_towards_enemy(
  game_state: &GameState,
  bot_position: &Position,
) -> Option<Direction> {
  if let Some(closest_enemy_position) = get_closest_enemy(game_state, bot_position) {
      if let Ok(next_move) = next_move_in_path(game_state, bot_position, &closest_enemy_position)
      {
          return Some(next_move);
      }
  }

  None
}

pub fn next_move_in_path(
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

pub fn find_shortest_path(
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
          if !visited[adjacent_pos.x][adjacent_pos.y] {
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

pub fn distance(from_pos_x: usize, from_pos_y: usize, to_pos_x: usize, to_pos_y: usize) -> usize {
  let x_distance = absolute(to_pos_x as isize - from_pos_x as isize);
  let y_distance = absolute(to_pos_y as isize - from_pos_y as isize);

  x_distance + y_distance
}

pub fn get_closest_enemy(game_state: &GameState, bot_position: &Position) -> Option<Position> {
  let mut closest_enemy: Option<Position> = None;

  for x in 0..game_state.map_width {
      for y in 0..game_state.map_height {
          if x != bot_position.x || y != bot_position.y {
              if is_bot(game_state, &Position { x, y }) {
                  match closest_enemy {
                      Some(Position {
                          x: closest_x,
                          y: closest_y,
                      }) if distance(closest_x, closest_y, bot_position.x, bot_position.y)
                          < distance(x, y, bot_position.x, bot_position.y) => {}
                      _ => closest_enemy = Some(Position { x, y }),
                  };
              }
          }
      }
  }

  closest_enemy
}

// Return whether we should gather a resource that's in an adjacent position to the bot's
// Return Some((resource_pos_x, resource_pos_y)) if we should gather the resource at that position
// Return None if we should not gather any resource around us
//
// Extra credit: only gather the resource if the bot has less than 9 energy
