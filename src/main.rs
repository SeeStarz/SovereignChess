mod engine;

use raylib::prelude::*;

fn main() {
    let (mut raylib_handle, thread) = raylib::init()
        .resizable()
        .size(640, 480)
        .title("Sovereign Chess")
        .vsync()
        .build();

    while !raylib_handle.window_should_close() {
        raylib_handle.draw(&thread, draw);
    }
}

fn draw(mut handle: RaylibDrawHandle) {
    handle.clear_background(Color::WHITE);
    handle.draw_rectangle(20, 20, 100, 100, Color::BLACK);
}
