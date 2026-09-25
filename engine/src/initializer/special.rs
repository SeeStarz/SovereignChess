use crate::{
    Coordinate, FactionID, faction,
    tile::{SpecialLayout, SpecialLayoutInput},
};

pub fn standard_layout() -> SpecialLayout {
    let inputs = [
        SpecialLayoutInput::new(
            Coordinate::new(7, 7),
            Coordinate::new(8, 8),
            FactionID::from(faction::White),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(5, 10),
            Coordinate::new(10, 5),
            FactionID::from(faction::Green),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(6, 9),
            Coordinate::new(9, 6),
            FactionID::from(faction::Ash),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(7, 10),
            Coordinate::new(8, 5),
            FactionID::from(faction::Cyan),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(4, 11),
            Coordinate::new(11, 4),
            FactionID::from(faction::Navy),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(5, 8),
            Coordinate::new(10, 7),
            FactionID::from(faction::Violet),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(5, 7),
            Coordinate::new(10, 8),
            FactionID::from(faction::Pink),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(4, 4),
            Coordinate::new(11, 11),
            FactionID::from(faction::Red),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(7, 5),
            Coordinate::new(8, 10),
            FactionID::from(faction::Orange),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(6, 6),
            Coordinate::new(9, 9),
            FactionID::from(faction::Slate),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(5, 5),
            Coordinate::new(10, 10),
            FactionID::from(faction::Yellow),
        ),
        SpecialLayoutInput::new(
            Coordinate::new(7, 8),
            Coordinate::new(8, 7),
            FactionID::from(faction::Black),
        ),
    ];

    SpecialLayout::new(&inputs).expect("Failed to initialize standard layout")
}
