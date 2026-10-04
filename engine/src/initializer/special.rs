use crate::shared::{Coordinate, FactionId, faction, tile};

pub fn standard() -> Vec<tile::SpecialPair> {
    vec![
        tile::SpecialPair::new(
            Coordinate::new(7, 7),
            Coordinate::new(8, 8),
            FactionId::from(faction::White),
        ),
        tile::SpecialPair::new(
            Coordinate::new(5, 10),
            Coordinate::new(10, 5),
            FactionId::from(faction::Green),
        ),
        tile::SpecialPair::new(
            Coordinate::new(6, 9),
            Coordinate::new(9, 6),
            FactionId::from(faction::Ash),
        ),
        tile::SpecialPair::new(
            Coordinate::new(7, 10),
            Coordinate::new(8, 5),
            FactionId::from(faction::Cyan),
        ),
        tile::SpecialPair::new(
            Coordinate::new(4, 11),
            Coordinate::new(11, 4),
            FactionId::from(faction::Navy),
        ),
        tile::SpecialPair::new(
            Coordinate::new(5, 8),
            Coordinate::new(10, 7),
            FactionId::from(faction::Violet),
        ),
        tile::SpecialPair::new(
            Coordinate::new(5, 7),
            Coordinate::new(10, 8),
            FactionId::from(faction::Pink),
        ),
        tile::SpecialPair::new(
            Coordinate::new(4, 4),
            Coordinate::new(11, 11),
            FactionId::from(faction::Red),
        ),
        tile::SpecialPair::new(
            Coordinate::new(7, 5),
            Coordinate::new(8, 10),
            FactionId::from(faction::Orange),
        ),
        tile::SpecialPair::new(
            Coordinate::new(6, 6),
            Coordinate::new(9, 9),
            FactionId::from(faction::Slate),
        ),
        tile::SpecialPair::new(
            Coordinate::new(5, 5),
            Coordinate::new(10, 10),
            FactionId::from(faction::Yellow),
        ),
        tile::SpecialPair::new(
            Coordinate::new(7, 8),
            Coordinate::new(8, 7),
            FactionId::from(faction::Black),
        ),
    ]
}

pub fn arena() -> Vec<tile::SpecialPair> {
    vec![
        tile::SpecialPair::new(
            Coordinate::new(4, 4),
            Coordinate::new(7, 7),
            FactionId::from(faction::White),
        ),
        tile::SpecialPair::new(
            Coordinate::new(6, 4),
            Coordinate::new(5, 7),
            FactionId::from(faction::Green),
        ),
        tile::SpecialPair::new(
            Coordinate::new(7, 3),
            Coordinate::new(4, 8),
            FactionId::from(faction::Cyan),
        ),
        tile::SpecialPair::new(
            Coordinate::new(8, 4),
            Coordinate::new(3, 7),
            FactionId::from(faction::Navy),
        ),
        tile::SpecialPair::new(
            Coordinate::new(4, 6),
            Coordinate::new(7, 5),
            FactionId::from(faction::Violet),
        ),
        tile::SpecialPair::new(
            Coordinate::new(4, 5),
            Coordinate::new(7, 6),
            FactionId::from(faction::Pink),
        ),
        tile::SpecialPair::new(
            Coordinate::new(3, 4),
            Coordinate::new(8, 7),
            FactionId::from(faction::Red),
        ),
        tile::SpecialPair::new(
            Coordinate::new(4, 3),
            Coordinate::new(7, 8),
            FactionId::from(faction::Orange),
        ),
        tile::SpecialPair::new(
            Coordinate::new(5, 4),
            Coordinate::new(6, 7),
            FactionId::from(faction::Yellow),
        ),
        tile::SpecialPair::new(
            Coordinate::new(7, 4),
            Coordinate::new(4, 7),
            FactionId::from(faction::Black),
        ),
    ]
}
