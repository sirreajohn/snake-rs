extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;

const BG_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (height, width) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake-rs", [to_coord_u32(height), to_coord_u32(width)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _device| {
            clear(BG_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
