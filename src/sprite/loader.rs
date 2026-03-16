use crate::sprite;
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

pub struct PositionedRectangle<T> {
    pub top: T,
    pub left: T,
    pub height: T,
    pub width: T,
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
            let mut image =
                Image::load_image(path).expect(&format!("Unable to load image {}", path));
            image.resize_nn(image.width * 2, image.height * 2);

            let texture = handle
                .load_texture_from_image(thread, &image)
                .expect(&format!("Unable to load texture {}", path));
            map.insert(sprite, texture);
        }
        Self { map }
    }

    pub fn get(&self, sprite: sprite::Prototype) -> (PositionedRectangle<f32>, &Texture2D) {
        (
            PositionedRectangle {
                top: 0.0,
                left: 0.0,
                height: 0.0,
                width: 0.0,
            },
            self.map.get(&sprite).unwrap(),
        )
    }
}
