use crate::{
    geometry::{FPosition, FRect, FSize},
    sprite,
};
use raylib::{
    RaylibHandle, RaylibThread,
    texture::{Image, Texture2D},
};
use std::{collections::HashMap, path::PathBuf};
use strum::IntoEnumIterator;

trait SpritePath {
    fn path(self) -> PathBuf;
}

impl SpritePath for sprite::Prototype {
    fn path(self) -> PathBuf {
        use sprite::Prototype::*;
        match self {
            KingOwnerMask => PathBuf::from("assets/img/king_base.png"),
            KingFactionMask => PathBuf::from("assets/img/king.png"),
            QueenOwnerMask => PathBuf::from("assets/img/queen_base.png"),
            QueenFactionMask => PathBuf::from("assets/img/queen.png"),
            RookOwnerMask => PathBuf::from("assets/img/rook_base.png"),
            RookFactionMask => PathBuf::from("assets/img/rook.png"),
            BishopOwnerMask => PathBuf::from("assets/img/bishop_base.png"),
            BishopFactionMask => PathBuf::from("assets/img/bishop.png"),
            KnightOwnerMask => PathBuf::from("assets/img/knight_base.png"),
            KnightFactionMask => PathBuf::from("assets/img/knight.png"),
            PawnOwnerMask => PathBuf::from("assets/img/pawn_base.png"),
            PawnFactionMask => PathBuf::from("assets/img/pawn.png"),
        }
    }
}

pub struct Manager {
    map: HashMap<sprite::Prototype, Texture2D>,
}

impl Manager {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut map = HashMap::new();
        for sprite in sprite::Prototype::iter() {
            let pathbuf = sprite.path();
            let path = pathbuf.to_str().expect("Path not a valid rust string");
            let image = Image::load_image(path).expect(&format!("Unable to load image {}", path));

            let texture = handle
                .load_texture_from_image(thread, &image)
                .expect(&format!("Unable to load texture {}", path));
            map.insert(sprite, texture);
        }
        Self { map }
    }

    pub fn get(&self, sprite: sprite::Prototype) -> (FRect, &Texture2D) {
        (
            FRect {
                position: FPosition { x: 0.0, y: 0.0 },
                size: FSize {
                    width: 16.0,
                    height: 16.0,
                },
            },
            self.map.get(&sprite).unwrap(),
        )
    }
}
