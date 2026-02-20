mod engine;

use crate::engine::{Coordinate, tile};
use raylib::prelude::*;

fn main() {
    let (mut raylib_handle, thread) = raylib::init()
        .resizable()
        .size(1080, 720)
        .title("Sovereign Chess")
        .vsync()
        .build();

    while !raylib_handle.window_should_close() {
        raylib_handle.draw(&thread, draw);
    }
}

fn draw(mut handle: RaylibDrawHandle) {
    handle.clear_background(Color::WHITE);
    for r in 0..16 {
        for c in 0..16 {
            let coordinate = Coordinate::new(r, c);
            let color = if let Some(special) = tile::Special::at(&coordinate) {
                Color::ROYALBLUE
            } else {
                if (r + c) % 2 == 0 {
                    Color::BROWN
                } else {
                    Color::LIGHTGRAY
                }
            };

            let size = 32;
            handle.draw_rectangle(size + c * size, size + r * size, size, size, color);
        }
    }
}
