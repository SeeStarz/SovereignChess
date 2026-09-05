use std::collections::HashSet;

use crate::engine::export::{Coordinate, Gamestate, Move, PieceExternal, faction, logic, piece};

pub use Click::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Click {
    BoardClick(Coordinate),
    PromotionClick(piece::Type),
    DefectionClick(faction::Color),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GamestateChange {
    pub gamestate: Gamestate,
    pub applied_move: Move,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterData {
    pub grabbed_piece: Option<PieceExternal>,
    pub promotion_at: Option<Coordinate>,
    pub gamestate_change: Option<GamestateChange>,
    pub valid_clicks: Vec<Click>,
}

impl AdapterData {
    pub fn new(valid_clicks: Vec<Click>) -> Self {
        Self {
            grabbed_piece: None,
            promotion_at: None,
            gamestate_change: None,
            valid_clicks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adapter {
    gamestate: Gamestate,
    valid_moves: Vec<Move>,
    clicks: Vec<Click>,
    data: AdapterData,
}

impl Adapter {
    pub fn new(gamestate: Gamestate) -> Self {
        let valid_moves = gamestate.moves();
        Self {
            clicks: Vec::new(),
            data: AdapterData::new(get_valid_first_clicks(&gamestate, &valid_moves)),
            valid_moves: valid_moves,
            gamestate,
        }
    }

    pub fn data(&self) -> AdapterData {
        self.data.clone()
    }

    pub fn clicks(&self) -> Vec<Click> {
        self.clicks.clone()
    }

    pub fn reset(&mut self) {
        self.clicks = Vec::new();
        self.data = AdapterData::new(get_valid_first_clicks(&self.gamestate, &self.valid_moves));
    }

    pub fn click(&mut self, click: Click) -> bool {
        if !self.data.valid_clicks.iter().any(|&c| c == click) {
            return false;
        }

        match self.clicks.len() {
            0 => self.first_click(click),
            1 => self.second_click(click),
            2 => self.third_click(click),
            3 => return false,
            _ => panic!("Click count should be 3 or less"),
        };

        true
    }

    fn first_click(&mut self, click: Click) {
        match click {
            BoardClick(coordinate) => {
                self.set_grabbed_piece_from_coordinate(coordinate);
            }
            DefectionClick(_faction) => {
                self.set_grabbed_piece_from_coordinate(
                    logic::find_current_player_king(&self.gamestate).coordinate,
                );
            }
            PromotionClick(_piece_type) => panic!("Promotion cannot be the first click"),
        };

        self.clicks.push(click);
        self.data.valid_clicks = get_valid_second_clicks(&self.gamestate, &self.valid_moves, click);
    }

    fn second_click(&mut self, click: Click) {
        assert!(self.clicks.len() == 1);

        let origin = match self.clicks[0] {
            Click::BoardClick(coordinate) => coordinate,
            Click::DefectionClick(_faction) => {
                logic::find_current_player_king(&self.gamestate).coordinate
            }
            Click::PromotionClick(_piece_type) => panic!("Promotion cannot be the first click"),
        };

        let Click::BoardClick(destination) = click else {
            panic!("Second click must be a board click");
        };

        let filtered_moves: Vec<Move> = self
            .valid_moves
            .iter()
            .filter(|&&m| {
                get_move_origin(&self.gamestate, m) == origin
                    && get_move_destination(&self.gamestate, m) == destination
            })
            .cloned()
            .collect();

        self.clicks.push(click);
        self.data.valid_clicks =
            get_valid_third_clicks(&self.valid_moves, [self.clicks[0], self.clicks[1]]);

        if filtered_moves.len() > 1 {
            // Promotion
            assert!(filtered_moves.iter().all(|&m| match m {
                Move::Promotion(_) => true,
                Move::RegimeChangePromotion(_) => true,
                _ => false,
            }));
            self.data.promotion_at = Some(destination);
        } else if filtered_moves.len() == 1 {
            // A move is made
            let applied_move = filtered_moves[0];
            let updated_gamestate = self.gamestate.apply_move(applied_move);
            self.data.gamestate_change = Some(GamestateChange {
                gamestate: updated_gamestate,
                applied_move,
            })
        }
    }

    fn third_click(&mut self, click: Click) {
        assert!(self.clicks.len() == 2);

        let Click::BoardClick(origin) = self.clicks[0] else {
            panic!("First click must be a board click for promotion");
        };

        let Click::BoardClick(destination) = self.clicks[1] else {
            panic!("Second click must be a board click");
        };

        let Click::PromotionClick(piece_type) = click else {
            panic!("Third click must be a promotion click");
        };

        self.clicks.push(click);

        let promotion_move = self
            .valid_moves
            .iter()
            .find(|&&m| {
                get_move_origin(&self.gamestate, m) == origin
                    && get_move_destination(&self.gamestate, m) == destination
                    && match m {
                        Move::Promotion(promotion_move) => promotion_move.piece_type == piece_type,
                        Move::RegimeChangePromotion(_promotion_move) => piece_type == piece::King,
                        _ => false,
                    }
            })
            .cloned()
            .expect("Cannot find promotion move");

        let updated_gamestate = self.gamestate.apply_move(promotion_move);
        self.data.gamestate_change = Some(GamestateChange {
            gamestate: updated_gamestate,
            applied_move: promotion_move,
        })
    }

    fn set_grabbed_piece_from_coordinate(&mut self, coordinate: Coordinate) {
        let grabbed_piece = logic::board_at_external(&self.gamestate, coordinate)
            .expect(&format!("Piece not found at {:?}", coordinate));
        self.data.grabbed_piece = Some(grabbed_piece);
    }
}

fn get_valid_first_clicks(gamestate: &Gamestate, valid_moves: &Vec<Move>) -> Vec<Click> {
    let mut valid_clicks = HashSet::new();

    for &chess_move in valid_moves {
        let move_origin = get_move_origin(gamestate, chess_move);
        valid_clicks.insert(Click::BoardClick(move_origin));

        if let Move::Defection(defection_move) = chess_move {
            valid_clicks.insert(Click::DefectionClick(defection_move.faction));
        }
    }

    valid_clicks.into_iter().collect()
}

fn get_valid_second_clicks(
    gamestate: &Gamestate,
    valid_moves: &Vec<Move>,
    click: Click,
) -> Vec<Click> {
    let mut valid_clicks = HashSet::new();

    let filtered_valid_moves: Vec<&Move> = match click {
        Click::PromotionClick(_piece_type) => panic!("Promotion click cannot be the first click"),
        Click::BoardClick(coordinate) => valid_moves
            .iter()
            .filter(|&&m| get_move_origin(gamestate, m) == coordinate)
            .collect(),
        Click::DefectionClick(faction) => valid_moves
            .iter()
            .filter(|&&m| {
                if let Move::Defection(defection_move) = m {
                    defection_move.faction == faction
                } else {
                    false
                }
            })
            .collect(),
    };

    for &chess_move in filtered_valid_moves {
        let move_destination = get_move_destination(gamestate, chess_move);
        valid_clicks.insert(Click::BoardClick(move_destination));
    }

    valid_clicks.into_iter().collect()
}

fn get_valid_third_clicks(valid_moves: &Vec<Move>, clicks: [Click; 2]) -> Vec<Click> {
    // Promotion must have board click as both earlier moves
    let Click::BoardClick(origin) = clicks[0] else {
        return Vec::new();
    };
    let Click::BoardClick(destination) = clicks[1] else {
        return Vec::new();
    };

    valid_moves
        .iter()
        .filter_map(|&m| {
            let Move::Promotion(promotion_move) = m else {
                return None;
            };
            if promotion_move.normal_move.origin == origin
                && promotion_move.normal_move.destination == destination
            {
                Some(Click::PromotionClick(promotion_move.piece_type))
            } else {
                None
            }
        })
        .collect()
}

fn get_move_origin(gamestate: &Gamestate, chess_move: Move) -> Coordinate {
    match chess_move {
        Move::NormalMove(normal_move) => normal_move.origin,
        Move::Castle(castle_move) => castle_move.king_move.origin,
        Move::Defection(_defection_move) => logic::find_current_player_king(gamestate).coordinate,
        Move::RegimeChangePromotion(promotion_move) => promotion_move.normal_move.origin,
        Move::Promotion(promotion_move) => promotion_move.normal_move.origin,
    }
}

fn get_move_destination(gamestate: &Gamestate, chess_move: Move) -> Coordinate {
    match chess_move {
        Move::NormalMove(normal_move) => normal_move.destination,
        Move::Castle(castle_move) => castle_move.king_move.destination,
        Move::Defection(defection_move) => defection_move
            .normal_move
            .and_then(|m| Some(m.destination))
            .unwrap_or_else(|| logic::find_current_player_king(gamestate).coordinate),
        Move::RegimeChangePromotion(promotion_move) => promotion_move.normal_move.destination,
        Move::Promotion(promotion_move) => promotion_move.normal_move.destination,
    }
}
