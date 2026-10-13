#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeatId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurnManager {
    player_count: u32,
    current_player: SeatId,
}

impl TurnManager {
    /// # Panics
    /// player_count must be >= 1
    pub fn new(player_count: u32) -> Self {
        if player_count < 1 {
            panic!("player_count must be at least 1");
        }

        TurnManager {
            player_count,
            current_player: SeatId(0),
        }
    }

    pub fn player_count(&self) -> u32 {
        self.player_count
    }

    pub fn current_player(&self) -> SeatId {
        self.current_player
    }

    pub fn next_turn(&self) -> TurnManager {
        TurnManager {
            player_count: self.player_count,
            current_player: SeatId((self.current_player.0 + 1) % self.player_count),
        }
    }
}
