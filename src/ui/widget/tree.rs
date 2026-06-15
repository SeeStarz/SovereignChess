use crate::{
    engine::export::{faction, piece},
    game::Data,
    geometry::{FPosition, FRect, FSize},
    sprite::{CompositeDraw, PieceSprite},
    ui::{
        input::Event,
        layout::{AxisSizingRequest, CardinalAnchor, FlexAnchor, FlexDirection, Positioning},
        widget::{
            ComputedWidgetNode, WidgetSizeRequest, WidgetSpec, WidgetSpecNode, component,
            ignore_input, no_render,
        },
    },
    util::Observer,
};
use glam::Vec2;
use raylib::{color::Color, math::Rectangle, prelude::RaylibDraw};
use std::{cell::RefCell, rc::Rc};

pub fn get(data_mutator: Rc<RefCell<Data>>, data_observer: Observer<Data>) -> ComputedWidgetNode {
    use AxisSizingRequest as Sizing;

    let mut last_selected = None;
    for piece_type in [
        piece::King,
        piece::Queen,
        piece::Rook,
        piece::Bishop,
        piece::Knight,
    ] {
        let promotion_selector = WidgetSpecNode {
            children: last_selected.map_or(vec![], |s| vec![s]),
            core: WidgetSpec {
                size_request: WidgetSizeRequest::new(Sizing::Fixed(64.0), Sizing::Fixed(64.0)),
                flex_direction: FlexDirection::Right,
                positioning: Positioning::Relative(
                    FPosition::new(64.0, 0.0),
                    CardinalAnchor::TopLeft,
                ),
                child_origin: CardinalAnchor::TopLeft,
                input_handler: Box::new({
                    let d = data_mutator.clone();
                    move |event, rect| match event {
                        Event::MousePressed(position)
                            if Rectangle::from(rect).check_collision_point_rec(position) =>
                        {
                            let mut d = d.borrow_mut();
                            if let Some(selected_piece_type) = d.selected_piece_type {
                                if selected_piece_type == piece_type {
                                    d.selected_piece_type = None;
                                } else {
                                    d.selected_piece_type = Some(piece_type);
                                }
                            } else {
                                d.selected_piece_type = Some(piece_type);
                            }
                            true
                        }
                        _ => false,
                    }
                }),
                render_function: Box::new({
                    let d = data_observer.clone();
                    move |handle, _thread, rect| {
                        if d.borrow()
                            .selected_piece_type
                            .is_none_or(|s| s != piece_type)
                        {
                            handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::GRAY);
                        } else {
                            handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::BLUE);
                        }
                        handle.draw_composite_pro(
                            &d.borrow().sprite_manager,
                            &PieceSprite {
                                piece_type,
                                faction: faction::White,
                                owner: Some(faction::White),
                            },
                            rect,
                            0.0,
                            Color::WHITE,
                        );
                    }
                }),
            },
        };

        last_selected = Some(promotion_selector);
    }

    let offsetter = WidgetSpecNode {
        children: vec![last_selected.unwrap()],
        core: WidgetSpec {
            size_request: WidgetSizeRequest::new(Sizing::Fixed(0.0), Sizing::Fixed(0.0)),
            positioning: Positioning::Absolute(
                FPosition::new(600.0, 300.0),
                CardinalAnchor::TopLeft,
            ),
            flex_direction: FlexDirection::Right,
            child_origin: CardinalAnchor::TopLeft,
            input_handler: Box::new(ignore_input),
            render_function: Box::new(no_render),
        },
    };

    let board = component::board::make_board(
        FRect {
            position: FPosition::default(),
            size: FSize::from(Vec2::new(32.0 * 16.0, 32.0 * 16.0)),
        },
        data_mutator.clone(),
    );

    let debug_toggle_button = component::debug::make_toggle_button(FRect {
        position: FPosition::new(600.0, 50.0),
        size: FSize::new(100.0, 100.0),
    });

    let root = WidgetSpecNode {
        children: vec![offsetter, debug_toggle_button, board],
        core: WidgetSpec {
            size_request: WidgetSizeRequest::new(Sizing::Fixed(0.0), Sizing::Fixed(0.0)),
            positioning: Positioning::Offset(FPosition::default(), FlexAnchor::Center),
            flex_direction: FlexDirection::Right,
            child_origin: CardinalAnchor::TopLeft,
            input_handler: Box::new(ignore_input),
            render_function: Box::new(no_render),
        },
    };

    root.compute()
}
