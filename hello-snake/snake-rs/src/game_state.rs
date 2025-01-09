use rand::Rng;
use std::collections::VecDeque;

const POINTS_PER_FRUIT: u32 = 10;
const MOVEMENT_INCREMENT: i32 = 10;
const SPEED_INCREMENT: u32 = 1;
const LIMIT_DISTANCE_TO_EAT: i32 = 10;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn inverse(&self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }

    fn near (&self, other: &Self, limit: i32) -> bool {
        (self.x - other.x).abs() < limit && (self.y - other.y).abs() < limit
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct GameState {
    pub fruit_position: Point,
    pub snake_position: Point,
    pub snake_speed: u32,
    pub snake_body: VecDeque<Point>,
    pub direction: Direction,
    pub score: u32,
    pub game_size: Point
}

impl GameState {
    pub fn new(weight: i32, height: i32) -> GameState {

        let size = Point::new(weight, height);
        let mut game_state = GameState {
            fruit_position: generate_random_position(&size),
            snake_position: generate_random_position(&size),
            snake_speed: 2,
            snake_body: VecDeque::<Point>::with_capacity(50),
            direction: Direction::LEFT,
            score: 0,
            game_size: size
        };

        game_state.snake_body.push_front(game_state.snake_position);
        game_state
    }

    pub fn move_snake(&mut self) {
        
        let mut x = self.snake_position.x;
        let mut y = self.snake_position.y;

        match self.direction {
            Direction::LEFT => x = (x - MOVEMENT_INCREMENT + self.game_size.x) % self.game_size.x,
            Direction::RIGHT => x = (x + MOVEMENT_INCREMENT) % self.game_size.x,
            Direction::UP => y = (y - MOVEMENT_INCREMENT + self.game_size.y) % self.game_size.y,
            Direction::DOWN => y = (y + MOVEMENT_INCREMENT) % self.game_size.y
        }

        self.snake_position.set(x, y);
        self.snake_body.push_front(self.snake_position);
        self.snake_body.pop_back();
    }

    pub fn is_crash(&self) -> bool {
        self.snake_body.iter().skip(1).any(|&p| p == self.snake_position)
    }

    pub fn is_fruit_eaten(&self) -> bool {
        self.fruit_position.near(&self.snake_position, LIMIT_DISTANCE_TO_EAT)
    }

    pub fn process_fruit(&mut self) {
        self.score += POINTS_PER_FRUIT;
        if let Some(last) = self.snake_body.back() {
            self.snake_body.push_back(*last);
        }
        self.snake_speed += SPEED_INCREMENT;
        self.fruit_position = generate_random_position(&self.game_size);
    }

    pub fn is_valid_direction(&self, direction: &Direction) -> bool {
        *direction != self.direction && *direction != self.direction.inverse()
    }
}

fn generate_random_position(game_size: &Point) -> Point {
    return Point::new(rand::thread_rng().gen_range(0..game_size.x), rand::thread_rng().gen_range(0..game_size.y));
}