#![allow(dead_code)]

mod engine;
mod geometry;
pub mod input;
pub mod render;
mod sprite;
mod ui;

fn main() {
    game::start();
}

pub mod game {
    use crate::{
        engine::export::{Coordinate, Gamestate, LegalMove},
        geometry::FPosition,
        input::Event,
        render::draw,
        sprite, ui,
        util::Observer,
    };
    use raylib::prelude::*;
    use std::{cell::RefCell, rc::Rc};

    pub struct Data {
        pub gamestate: Gamestate,
        pub legal_moves: Vec<LegalMove>,
        pub selected_square: Option<Coordinate>,
        pub sprite_manager: sprite::Manager,
    }

    pub fn start() {
        let (mut raylib_handle, thread) = raylib::init()
            .resizable()
            .size(1080, 720)
            .title("Sovereign Chess")
            .vsync()
            .build();

        let data_mutator = Rc::new(RefCell::new({
            let gamestate = Gamestate::new();
            Data {
                gamestate,
                legal_moves: gamestate.moves(),
                selected_square: None,
                sprite_manager: sprite::Manager::new(&mut raylib_handle, &thread),
            }
        }));

        let data_observer = Observer::from(data_mutator.clone());
        let mut widget_tree = ui::widget::tree::get(data_mutator, data_observer);

        while !raylib_handle.window_should_close() {
            let events = {
                let mut events = Vec::new();
                {
                    let mouse_pressed =
                        raylib_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
                    if mouse_pressed {
                        let position = FPosition::from(raylib_handle.get_mouse_position());
                        events.push(Event::MousePressed(position));
                    }
                }
                events
            };

            events.iter().for_each(|&event| {
                widget_tree.handle_input(event);
            });

            //// Below is the preferred way of doing things, but is still broken (double END_DRAWING) as per version 5.5.1
            // raylib_handle.draw(&thread, |handle| ...);
            //// Working version
            {
                let draw_handle = raylib_handle.begin_drawing(&thread);
                draw(draw_handle, &thread, &widget_tree);
            }
        }
    }
}

mod util {
    use std::{cell::RefCell, ops::Deref, rc::Rc};

    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct Observer<T>(Rc<RefCell<T>>);
    impl<T> From<Rc<RefCell<T>>> for Observer<T> {
        fn from(value: Rc<RefCell<T>>) -> Self {
            Self(value)
        }
    }

    impl<T> Observer<T> {
        /// # Panics
        /// Panics if the value is currently mutably borrowed.
        pub fn borrow(&self) -> impl Deref<Target = T> {
            self.0.borrow()
        }
    }

    impl<T> Clone for Observer<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
}
