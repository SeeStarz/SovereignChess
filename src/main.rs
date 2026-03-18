#![allow(dead_code)]

mod engine;
mod geometry;
pub mod render;
mod sprite;
mod ui;

fn main() {
    game::start();
}

pub mod game {
    use crate::{
        engine::export::{Coordinate, Gamestate, Move},
        geometry::{FPosition, FRect, FSize},
        render::{draw, draw_game},
        sprite,
        ui::{Layout, WidgetIntent, widget},
        util::Observer,
    };
    use glam::Vec2;
    use raylib::prelude::*;
    use std::{cell::RefCell, rc::Rc};

    pub struct Data {
        pub gamestate: Gamestate,
        pub legal_moves: Vec<Move>,
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
                legal_moves: gamestate.get_moves(),
                selected_square: None,
                sprite_manager: sprite::Manager::new(&mut raylib_handle, &thread),
            }
        }));

        let data_observer = Observer::from(data_mutator.clone());

        let widget_tree = {
            let board = WidgetIntent {
                children: Vec::new(),
                layout: Layout::Relative(FRect {
                    position: FPosition::default(),
                    size: FSize::from(Vec2::new(32.0 * 16.0, 32.0 * 16.0)),
                }),
                input_handler: Box::new(widget::ignore_input),
                render_function: Box::new({
                    let d = data_observer.clone();
                    move |handle, thread, rect| {
                        draw_game(handle, thread, rect, &d.borrow());
                    }
                }),
            };
            board.compute(FRect {
                position: FPosition::from(Vec2::new(32.0, 32.0)),
                size: FSize::default(),
            })
        };

        while !raylib_handle.window_should_close() {
            if raylib_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let row = (raylib_handle.get_mouse_y() / 32) - 1;
                let col = (raylib_handle.get_mouse_x() / 32) - 1;
                if let Some(coordinate1) = Coordinate::new(row, col) {
                    let selected_square = data_mutator.borrow().selected_square;
                    if let Some(coordinate2) = selected_square {
                        let attempted_move = Move {
                            origin: coordinate2,
                            destination: coordinate1,
                        };

                        let is_legal = data_mutator
                            .borrow()
                            .legal_moves
                            .iter()
                            .find(|&&x| x == attempted_move)
                            .is_some();
                        if is_legal {
                            let gs = data_mutator.borrow().gamestate.apply_move(attempted_move);
                            data_mutator.borrow_mut().gamestate = gs;
                            let moves = data_mutator.borrow().gamestate.get_moves();
                            data_mutator.borrow_mut().legal_moves = moves;
                        }
                        data_mutator.borrow_mut().selected_square = None;
                    } else {
                        let is_piece_here = data_mutator
                            .borrow()
                            .gamestate
                            .pieces()
                            .find(|p| p.coordinate == coordinate1)
                            .is_some();
                        if is_piece_here {
                            data_mutator.borrow_mut().selected_square = Some(coordinate1);
                        }
                    }
                }
            }

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
