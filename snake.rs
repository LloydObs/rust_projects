use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw:: draw_block;

const snakeColor : Color = [0.00, 0.80, 0.00, 1.0];
#[derive(Copy, Clone, PartialEq)]
pub enum snakeDirection{
    Up,
    Down, 
    Left,
    Right,
}

impl snakeDirection{
    pub fn opposite(&self) -> snakeDirection {
        match *self{
            snakeDirection::Up => snakeDirection::Down,
            snakeDirection::Down => snakeDirection::Up,
            snakeDirection::Left => snakeDirection::Right,
            snakeDirection::Right => snakeDirection::Left,
        }
    }


}

#[derive(Debug, Clone)]
struct returnBlock {
    x: i32,
    y: i32,
}

pub struct snakeDetails {
    direction: snakeDirection,
    body: LinkedList<returnBlock>,
    tail: Option<returnBlock>,
}

impl snakeDetails {
    pub fn new (x: i32, y: i32) -> snakeDetails{
        let mut body: LinkedList<returnBlock> = LinkedList::new(); 
        body.push_back(returnBlock{
            x: x + 2,
            y,
        });
        body.push_back(returnBlock {
            x: x + 1,
            y,
        });
        body.push_back(returnBlock {
            x,
            y,
        });

        snakeDetails { direction: snakeDirection::Right, body, tail: None,}
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body{
            draw_block(snakeColor, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }
    
    pub fn move_forward(&mut self, dir: Option<snakeDirection>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            snakeDirection::Up => returnBlock {
                x: last_x,
                y: last_y -1,
            },
            snakeDirection::Down => returnBlock{
                x: last_x,
                y: last_y + 1,
            },
            snakeDirection::Left => returnBlock { 
                x: last_x - 1,
                y: last_y,
            },
            snakeDirection::Right => returnBlock{
                x: last_x + 1,
                y:last_y,
            }
        };
        self.body.push_front(new_block);
        let removeBlock = self.body.pop_back().unwrap();
        self.tail = Some(removeBlock);
    }

    pub fn headDirection(&self) -> snakeDirection {
        self.direction
    }
    pub fn nextHead(&self, dir: Option<snakeDirection>) -> (i32, i32) {
        let (headXValue, headYValue): (i32, i32) = self.head_position();

        let mut movingDir = self.direction;
        match dir {
            Some(d) => movingDir = d,
            None => {}
        }

        match movingDir{
            snakeDirection::Up => (headXValue, headYValue - 1),
            snakeDirection::Down => (headXValue, headYValue + 1),
            snakeDirection::Left => (headXValue - 1, headYValue),
            snakeDirection::Right => (headXValue + 1, headYValue),
        }
    }
        

        pub fn restoreTail(&mut self) {
            let block = self.tail.clone().unwrap();
            self.body.push_back(block);
        }

        pub fn overLapTail(&self, x:i32, y: i32) -> bool {

            let mut ch = 0;
            for block in &self.body {
                if x == block.x && y == block.y {
                    return true;
                }

                ch += 1;
                if ch == self.body.len() - 1{
                    break;
                }

            }
            return false;
        }
}

