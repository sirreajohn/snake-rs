use piston_window::{Context, G2d, types::Color};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 0.90];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y: y });
        body.push_back(Block { x: x + 1, y: y });
        body.push_back(Block { x: x, y: y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn get_head_pos(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (x, y) = self.get_head_pos();

        let new_block = match self.direction {
            Direction::Down => Block { x: x, y: y + 1 },
            Direction::Up => Block { x: x, y: y - 1 },
            Direction::Left => Block { x: x - 1, y: y },
            Direction::Right => Block { x: x + 1, y: y },
        };

        self.body.push_front(new_block);
        let last_block = self.body.pop_back().unwrap();
        self.tail = Some(last_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (x, y) = self.get_head_pos();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => (),
        }

        match moving_dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    pub fn restore_tail(&mut self) {
        let back = self.tail.clone().unwrap();
        self.body.push_back(back);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;

            if ch == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}
