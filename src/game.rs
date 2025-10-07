use piston_window::types::Color;
use piston_window::*;

use rand::{Rng, rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.60, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.0, 0.0, 1.0];

const MOVE_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 0.5;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 3,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir == None {
            return;
        }

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir)
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // draw borders
        draw_rectangle(BORDER_COLOR, 0, 0, self.width - 1, 1, con, g); // top
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g); // bot
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height - 1, con, g); // left
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height - 1, con, g); // right

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, deltatime: f64) {
        self.waiting_time += deltatime;
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart()
            }
        }

        if !self.food_exists {
            self.add_food()
        }

        if self.waiting_time > MOVE_PERIOD {
            self.update_snake(None)
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.get_head_pos();
        if self.food_exists && (head_x == self.food_x) && (head_y == self.food_y) {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng_gen = rng();

        let mut new_food_x = rng_gen.random_range(1..self.width - 1);
        let mut new_food_y = rng_gen.random_range(1..self.height - 1);

        while self.snake.overlap_tail(new_food_x, new_food_y) {
            new_food_x = rng_gen.random_range(1..self.width - 1);
            new_food_y = rng_gen.random_range(1..self.height - 1);
        }

        self.food_x = new_food_x;
        self.food_y = new_food_y;

        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food_exists = true;
        self.food_x = 3;
        self.food_y = 4;
        self.game_over = false;
        self.waiting_time = 0.0;
    }
}
