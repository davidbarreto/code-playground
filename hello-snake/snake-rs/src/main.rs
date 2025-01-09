mod game_state;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use game_state::Direction;

use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Rect};
use ggez::event::{self, EventHandler};

lazy_static! {
    static ref DIRECTIONS: HashMap<KeyCode, Direction> = {
        let mut map = HashMap::new();
        map.insert(KeyCode::Up, Direction::UP);
        map.insert(KeyCode::Down, Direction::DOWN);
        map.insert(KeyCode::Left, Direction::LEFT);
        map.insert(KeyCode::Right, Direction::RIGHT);
        map
    };
}

const RECT_SIZE:f32 = 10.0;
const GAME_WIDTH:i32 = 800;
const GAME_HEIGHT:i32 = 600;

struct Game {
    game_over: bool,
    game_state: game_state::GameState
}

impl Game {
    fn new(_ctx: &mut Context) -> GameResult<Game> {
        let game = Game {
            game_over: false,
            game_state: game_state::GameState::new(GAME_WIDTH, GAME_HEIGHT)
        };
        Ok(game)
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        if _ctx.time.check_update_time(self.game_state.snake_speed) {
            if self.game_state.is_crash() {
                self.game_over = true;
            }

            if !self.game_over {
                if self.game_state.is_fruit_eaten() {
                    self.game_state.process_fruit();
                }

                self.game_state.move_snake();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Draw code here...

        //Draw the snake
        for point in &self.game_state.snake_body {
            // Again we set the color (in this case an orangey color)
            // and then draw the Rect that we convert that Segment's position into
            let position = point.to_tuple();
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(Rect::new(position.0, position.1, RECT_SIZE, RECT_SIZE))
                    .color(Color::GREEN)
            );
        }

        //Draw the fruit
        let position = self.game_state.fruit_position.to_tuple();
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(position.0, position.1, RECT_SIZE, RECT_SIZE))
                .color(Color::RED)
        );

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {

        if let Some(keycode) = input.keycode {
            if let Some(direction) = DIRECTIONS.get(&keycode) {
                if self.game_state.is_valid_direction(direction) {
                    self.game_state.direction = *direction;
                }
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("snake_rs", "David").build()?;
    let my_game = Game::new(&mut ctx)?;
    ctx.gfx.set_drawable_size(GAME_WIDTH as f32, GAME_HEIGHT as f32)?;
    event::run(ctx, event_loop, my_game)
}