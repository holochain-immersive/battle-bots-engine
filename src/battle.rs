use rand::Rng;
use ruscii::{
    app::{App, Config, State},
    drawing::{Pencil, RectCharset},
    gui::FPSCounter,
    keyboard::{Key, KeyEvent},
    spatial::Vec2,
    terminal::{Color, Window},
};

use crate::{
    bot::{Bot, BotStrategy, ColorConfig},
    constants::*,
    direction::Direction,
    resource::Resource,
    state::{from_matrix, state_to_matrix, GameCell, GameState, Position},
};

pub struct Battle {
    pub state: GameState,
    pub colors: Vec<ColorConfig>,
}

impl Battle {
    pub fn new(colors: Vec<ColorConfig>) -> Battle {
        let mut map = [[GameCell::Empty; MAP_HEIGHT]; MAP_WIDTH];

        for color_config in colors.iter() {
            for _ in 0..color_config.number_of_bots {
                if let Some(Position { x, y }) = find_empty_position(&map) {
                    map[x][y] = GameCell::Bot(Bot::new(color_config.color));
                }
            }
        }

        let state = from_matrix(map);

        Battle { state, colors }
    }

    pub fn run(&mut self) {
        let mut app = App::config(Config::new().fps(2));

        let mut fps_counter = FPSCounter::new();

        app.run(|app_state: &mut State, window: &mut Window| {
            for key_event in app_state.keyboard().last_key_events() {
                match key_event {
                    KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                    KeyEvent::Pressed(Key::Q) => app_state.stop(),
                    _ => (),
                }
            }

            fps_counter.update();
            self.update();

            let mut pencil = Pencil::new(window.canvas_mut());

            pencil
                .set_origin(Vec2::xy(1 as usize, 1 as usize))
                .set_foreground(Color::Grey)
                .draw_rect(
                    &RectCharset::double_lines(),
                    Vec2::xy(-1 as isize, -1 as isize),
                    Vec2::xy(MAP_WIDTH * 3 + 2, MAP_HEIGHT * 3 + 2),
                );

            let map = state_to_matrix(self.state.clone());

            for x in 0..MAP_WIDTH {
                for y in 0..MAP_HEIGHT {
                    if let GameCell::Bot(bot) = map[x][y] {
                        pencil.set_foreground(bot.color);
                        pencil.draw_char(
                            format!("{}", bot.energy).as_str().chars().next().unwrap(),
                            Vec2::xy(x * 3, (MAP_HEIGHT - 1 - y) * 3),
                        );
                        let bot_down = Vec2::xy(
                            (x as i32) * 3,
                            ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3 + 1,
                        );
                        let bot_up = Vec2::xy(
                            (x as i32) * 3,
                            ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3 - 1,
                        );
                        let bot_left = Vec2::xy(
                            (x as i32) * 3 - 1,
                            ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3,
                        );
                        let bot_right = Vec2::xy(
                            (x as i32) * 3 + 1,
                            ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3,
                        );

                        if !bot.is_shield_destroyed()
                            && bot.shield_direction.eq(&bot.chainsaw_direction)
                        {
                            match bot.shield_direction {
                                Direction::Down => pencil.draw_char('⤈', bot_down),
                                Direction::Up => pencil.draw_char('⤉', bot_up),
                                Direction::Left => pencil.draw_char('⇷', bot_left),
                                Direction::Right => pencil.draw_char('⇸', bot_right),
                            };
                        } else {
                            if !bot.is_shield_destroyed() {
                                match bot.shield_direction {
                                    Direction::Down => pencil.draw_char('—', bot_down),
                                    Direction::Up => pencil.draw_char('—', bot_up),
                                    Direction::Left => pencil.draw_char('|', bot_left),
                                    Direction::Right => pencil.draw_char('|', bot_right),
                                };
                            }
                            match bot.chainsaw_direction {
                                Direction::Down => pencil.draw_char('↓', bot_down),
                                Direction::Up => pencil.draw_char('↑', bot_up),
                                Direction::Left => pencil.draw_char('←', bot_left),
                                Direction::Right => pencil.draw_char('→', bot_right),
                            };
                        }
                    } else if let GameCell::Resource(resource) = &map[x][y] {
                        pencil.set_foreground(Color::White);
                        pencil.draw_center_text(
                            format!("{}", resource.energy_gain).as_str(),
                            Vec2::xy(x * 3, (MAP_HEIGHT - 1 - y) * 3),
                        );
                    }
                }
            }
        });
    }

    fn strategy_for(&self, color: Color) -> Option<BotStrategy> {
        self.colors
            .iter()
            .find(|c| c.color == color)
            .map(|c| c.strategy)
    }

    fn update(&mut self) {

        // START HERE!
    }
}

fn find_empty_position(map: &[[GameCell; MAP_HEIGHT]; MAP_WIDTH]) -> Option<Position> {
    let mut rng = rand::thread_rng();

    loop {
        let x: usize = rng.gen_range(0..MAP_WIDTH);
        let y: usize = rng.gen_range(0..MAP_HEIGHT);

        if let GameCell::Empty = map[x][y] {
            return Some(Position { x, y });
        }
    }
}
