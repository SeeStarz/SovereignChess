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
    fn for_each_part<T>(&self, f: &mut T)
    where
        T: FnMut(CompositeSpritePart);
}

pub trait CompositeDraw<T> {
    fn draw_composite(
        &mut self,
        asset_manager: &Manager,
        composite: &T,
        x: i32,
        y: i32,
        tint: Color,
    ) where
        T: CompositeSprite;
}

impl<'a, T> CompositeDraw<T> for RaylibDrawHandle<'a> {
    fn draw_composite(
        &mut self,
        asset_manager: &Manager,
        composite: &T,
        x: i32,
        y: i32,
        tint: Color,
    ) where
        T: CompositeSprite,
    {
        composite.for_each_part(&mut |part| {
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
        });
    }
}

pub struct PieceSprite {
    pub piece_type: piece::Type,
    pub faction: faction::Color,
    pub owner: Option<faction::Color>,
}

impl CompositeSprite for PieceSprite {
    fn for_each_part<T>(&self, f: &mut T)
    where
        T: FnMut(CompositeSpritePart),
    {
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

        // Body sprite
        f(CompositeSpritePart {
            sprite: faction_mask,
            offset_x: 0,
            offset_y: 0,
            tint: self.faction.to_color(),
        });

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

        // Lower part sprite
        f(CompositeSpritePart {
            sprite: owner_mask,
            offset_x: 0,
            offset_y: 0,
            tint: self.owner.map_or(Color::GRAY.alpha(0.5), |c| c.to_color()),
        });
    }
}
