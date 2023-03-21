use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_cord(game_coordinates: i32) -> f64{
    (game_coordinates as f64) * BLOCK_SIZE
}

pub fn to_cord32(game_coordinates: i32) -> u32{
    to_cord(game_coordinates) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d){
    let blockXValue = to_cord(x);
    let blockYValue = to_cord(y);

    rectangle(
        color,
        [blockXValue, blockYValue, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,

) {

    let boardXValue = to_cord(x);
    let boardYValue = to_cord(y);

    rectangle(
        color, 
        [boardXValue, boardYValue, BLOCK_SIZE * (width as f64),BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    );
}