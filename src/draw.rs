use piston_window::{Context, G2d, rectangle, types::Color};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(coord: i32) -> f64 {
    (coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(coord: i32) -> u32 {
    to_coord(coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    )
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
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            (width as f64) * BLOCK_SIZE,
            (height as f64) * BLOCK_SIZE,
        ],
        con.transform,
        g,
    )
}
