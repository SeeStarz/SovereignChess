use glam::Vec2;
use raylib::math::Rectangle;

use crate::{
    engine::export::{Coordinate, Move},
    game::Data,
    geometry::{FPosition, FRect},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    MousePressed(FPosition),
    _ExtensibilityMarker(private::Private),
}

mod private {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Private;
}

pub fn handle_board_input(event: Event, rect: FRect, data: &mut Data) -> bool {
    let Event::MousePressed(click_position) = event else {
        return false;
    };
    if !Rectangle::from(rect).check_collision_point_rec(click_position) {
        return false;
    }

    let tile_size = Vec2::from(rect.size) / 16.0;
    let position = (Vec2::from(click_position) - Vec2::from(rect.position)) / tile_size;
    let Some(coordinate1) = Coordinate::new(position.y as i32, position.x as i32) else {
        return false;
    };

    if let Some(coordinate2) = data.selected_square {
        let attempted_move = Move {
            origin: coordinate2,
            destination: coordinate1,
        };

        if data.legal_moves.iter().any(|&x| x == attempted_move) {
            data.gamestate = data.gamestate.apply_move(attempted_move);
            data.legal_moves = data.gamestate.get_moves();
        }
        data.selected_square = None;
    } else {
        if data
            .gamestate
            .pieces()
            .find(|p| p.coordinate == coordinate1)
            .is_some()
        {
            data.selected_square = Some(coordinate1);
        }
    }

    true
}
