use crate::ToColor;
use raylib::{
    color::Color,
    core::drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    engine::export::{faction, piece},
    sprite::{self, Manager},
};

pub struct CompositeSpritePart {
    pub sprite: sprite::Prototype,
    pub offset_x: i32,
    pub offset_y: i32,
    pub tint: Color,
}

pub trait CompositeSprite {
    fn get_parts(&self) -> Vec<CompositeSpritePart>;
}

pub trait CompositeDraw {
    fn draw_composite(
        &mut self,
        asset_manager: &Manager,
        composite: impl CompositeSprite,
        x: i32,
        y: i32,
        tint: Color,
    );
}

impl<'a> CompositeDraw for RaylibDrawHandle<'a> {
    fn draw_composite(
        &mut self,
        asset_manager: &Manager,
        composite: impl CompositeSprite,
        x: i32,
        y: i32,
        tint: Color,
    ) {
        for part in composite.get_parts() {
            let (_texture_rect, texture) = asset_manager.get(part.sprite);
            self.draw_texture(
                texture,
                x + part.offset_x,
                y + part.offset_y,
                Color {
                    r: (tint.r as u16 * part.tint.r as u16 / 255) as u8,
                    g: (tint.g as u16 * part.tint.g as u16 / 255) as u8,
                    b: (tint.b as u16 * part.tint.b as u16 / 255) as u8,
                    a: (tint.a as u16 * part.tint.a as u16 / 255) as u8,
                },
            );
        }
    }
}

pub struct PieceSprite {
    pub piece_type: piece::Type,
    pub faction: faction::Color,
    pub owner: Option<faction::Color>,
}

impl CompositeSprite for PieceSprite {
    fn get_parts(&self) -> Vec<CompositeSpritePart> {
        let faction_mask = {
            use piece::Type::*;
            use sprite::Prototype::*;
            match self.piece_type {
                King => KingFactionMask,
                Queen => QueenFactionMask,
                Rook => RookFactionMask,
                Bishop => BishopFactionMask,
                Knight => KnightFactionMask,
                Pawn => PawnFactionMask,
            }
        };

        let owner_mask = {
            use piece::Type::*;
            use sprite::Prototype::*;
            match self.piece_type {
                King => KingOwnerMask,
                Queen => QueenOwnerMask,
                Rook => RookOwnerMask,
                Bishop => BishopOwnerMask,
                Knight => KnightOwnerMask,
                Pawn => PawnOwnerMask,
            }
        };

        let body_sprite = CompositeSpritePart {
            sprite: faction_mask,
            offset_x: 0,
            offset_y: 0,
            tint: self.faction.to_color(),
        };

        let bottom_sprite = CompositeSpritePart {
            sprite: owner_mask,
            offset_x: 0,
            offset_y: 0,
            tint: self.owner.map_or(Color::GRAY.alpha(0.5), |c| c.to_color()),
        };

        vec![body_sprite, bottom_sprite]
    }
}
