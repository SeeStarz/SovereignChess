mod engine;
mod sprite;
mod ui;

use std::{cell::RefCell, rc::Rc};

use crate::{
    engine::export::{Coordinate, Gamestate, Move, faction, tile},
    sprite::{CompositeDraw, PieceSprite},
    ui::{
        WidgetIntent,
        widget::{self, ComputedWidget},
    },
};
use glam::IVec2;
use raylib::prelude::*;

struct Data {
    gamestate: Gamestate,
    legal_moves: Vec<Move>,
    selected_square: Option<Coordinate>,
    sprite_manager: sprite::Manager,
}

fn main() {
    let (mut raylib_handle, thread) = raylib::init()
        .resizable()
        .size(1080, 720)
        .title("Sovereign Chess")
        .vsync()
        .build();

    let data = Rc::new(RefCell::new({
        let gamestate = Gamestate::new();
        Data {
            gamestate,
            legal_moves: gamestate.get_moves(),
            selected_square: None,
            sprite_manager: sprite::Manager::new(&mut raylib_handle, &thread),
        }
    }));

    let widget_tree = {
        let board = WidgetIntent {
            children: Vec::new(),
            layout: ui::Layout::Relative(IRect {
                position: IPosition::default(),
                size: ISize::from(IVec2::new(32 * 16, 32 * 18)),
            }),
            input_handler: Box::new(widget::ignore_input),
            render_function: Box::new({
                let d = data.clone();
                move |handle, thread, _rect| {
                    draw_board(handle, thread, &d.borrow());
                    draw_pieces(handle, thread, &d.borrow());
                    draw_legal_moves(handle, thread, &d.borrow());
                }
            }),
        };
        board.compute(IRect {
            position: IPosition::from(IVec2::new(32, 32)),
            size: ISize::default(),
        })
    };

    while !raylib_handle.window_should_close() {
        if raylib_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let row = (raylib_handle.get_mouse_y() / 32) - 1;
            let col = (raylib_handle.get_mouse_x() / 32) - 1;
            if let Some(coordinate1) = Coordinate::new(row, col) {
                let selected_square = data.borrow().selected_square;
                if let Some(coordinate2) = selected_square {
                    let attempted_move = Move {
                        origin: coordinate2,
                        destination: coordinate1,
                    };

                    let is_legal = data
                        .borrow()
                        .legal_moves
                        .iter()
                        .find(|&&x| x == attempted_move)
                        .is_some();
                    if is_legal {
                        let gs = data.borrow().gamestate.apply_move(attempted_move);
                        data.borrow_mut().gamestate = gs;
                        let moves = data.borrow().gamestate.get_moves();
                        data.borrow_mut().legal_moves = moves;
                    }
                    data.borrow_mut().selected_square = None;
                } else {
                    let is_piece_here = data
                        .borrow()
                        .gamestate
                        .pieces()
                        .find(|p| p.coordinate == coordinate1)
                        .is_some();
                    if is_piece_here {
                        data.borrow_mut().selected_square = Some(coordinate1);
                    }
                }
            }
        }

        //// Below is the preferred way of doing things, but is still broken (double END_DRAWING) as per version 5.5.1
        // raylib_handle.draw(&thread, |handle| draw(handle, &thread, &data));
        //// Working version
        {
            let draw_handle = raylib_handle.begin_drawing(&thread);
            draw(draw_handle, &thread, &widget_tree);
        }
    }
}

fn draw(mut handle: RaylibDrawHandle, thread: &RaylibThread, widget_tree: &ComputedWidget) {
    handle.clear_background(Color::BLACK);
    widget_tree
        .iter()
        .for_each(|w| (w.render_function)(&mut handle, thread, &w.rect));
}

fn draw_board(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, _data: &Data) {
    for r in 0..16 {
        for c in 0..16 {
            let coordinate = Coordinate::new_unchecked(r, c);
            let color = if let Some(special) = tile::Special::at(coordinate) {
                special.faction.to_color()
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

trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for faction::Color {
    fn to_color(self) -> Color {
        use faction::Color::*;
        match self {
            White => Color::WHITE,
            Pink => Color::PINK,
            Slate => Color::SLATEGRAY,
            Red => Color::RED,
            Orange => Color::ORANGE,
            Yellow => Color::YELLOW,
            Green => Color::GREEN,
            Cyan => Color::CYAN,
            Navy => Color::NAVY,
            Ash => Color::GRAY,
            Violet => Color::VIOLET,
            Black => Color::BLACK.brightness(0.2),
        }
    }
}

fn draw_pieces(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, data: &Data) {
    let size = 32;
    for piece in data.gamestate.pieces() {
        let sprite = PieceSprite {
            piece_type: piece.piece_type,
            faction: piece.faction,
            owner: piece.owner,
        };

        handle.draw_composite(
            &data.sprite_manager,
            &sprite,
            piece.coordinate.col() as i32 * size + size,
            piece.coordinate.row() as i32 * size + size,
            Color::WHITE,
        );
    }
}

fn draw_legal_moves(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, data: &Data) {
    let Some(selected_square) = data.selected_square else {
        return;
    };

    let size = 32;
    for move_ in data
        .legal_moves
        .iter()
        .filter(|&&mv| mv.origin == selected_square)
    {
        handle.draw_circle(
            (move_.destination.col() as i32 + 1) * size + size / 2,
            (move_.destination.row() as i32 + 1) * size + size / 2,
            size as f32 / 3.0,
            Color::BLUE.alpha(0.25),
        );
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct IPosition(pub IVec2);
impl From<IVec2> for IPosition {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}
impl From<IPosition> for IVec2 {
    fn from(value: IPosition) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct ISize {
    width: i32,
    height: i32,
}
impl From<IVec2> for ISize {
    fn from(value: IVec2) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}
impl From<ISize> for IVec2 {
    fn from(value: ISize) -> Self {
        IVec2 {
            x: value.width,
            y: value.height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct IRect {
    position: IPosition,
    size: ISize,
}
