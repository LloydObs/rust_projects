use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{snakeDirection, snakeDetails};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: snakeDetails,

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
        Game { snake: snakeDetails::new(2,2),
             food_exists: true,
             food_x: 6,
             food_y: 4,
             width,
             height,
             game_over: false,
             waiting_time: 0.0 }
    }
    pub fn keyPressed(&mut self, key: Key){
        if self.game_over {
            return;
        }

        let dir = match key{ 
            Key::Up => Some(snakeDirection::Up),
            Key::Down => Some(snakeDirection::Down),
            Key::Left => Some(snakeDirection::Left),
            Key::Right => Some(snakeDirection::Right),
            _ => Some(self.snake.headDirection()),
        };

        if dir.unwrap() == self.snake.headDirection().opposite() {
            return;

        }
        self.updateSnake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        self.snake.draw(con, g);

        if self.food_exists{
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g)
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);


        if self.game_over{
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g)
        }
    }

    pub fn update(&mut self, delta_time: f64){
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.addFood();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.updateSnake(None);
        }
    }

    fn checkEating(&mut self){
        let (headXValue, headYValue): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == headXValue && self.food_y == headYValue{
            self.food_exists = false;
            self.snake.restoreTail();
        }
        
    }
    fn checkSnakeState(&self, dir: Option<snakeDirection>) -> bool {
        let (nextXVal, nextYVal) = self.snake.nextHead(dir);

        if self.snake.overLapTail(nextXVal, nextYVal) {
            return false;
        }
        nextXVal > 0 && nextYVal > 0 && nextXVal < self.width - 1 && nextYVal < self.height -1
    }

    fn addFood(&mut self){
        let mut rng = thread_rng();

        let mut newXVal = rng.gen_range(1..self.width - 1 );
        let mut newYVal = rng.gen_range(1..self.height -1 );
        while self.snake.overLapTail(newXVal, newYVal){
            newXVal = rng.gen_range(1..self.width - 1);
            newYVal = rng.gen_range(1..self.height -1);
        }

        self.food_x = newXVal;
        self.food_y = newYVal;
        self.food_exists = true;

    }

    fn updateSnake(&mut self, dir: Option<snakeDirection>) {
        if self.checkSnakeState(dir) {
            self.snake.move_forward(dir);
            self.checkEating();
        }else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
    fn restart(&mut self){
        self.snake = snakeDetails::new(2,2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;

    }
}
    


