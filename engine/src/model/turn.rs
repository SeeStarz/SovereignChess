#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurnId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurnManager {
    player_count: u32,
    current_player: TurnId,
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
            current_player: TurnId(0),
        }
    }

    pub fn player_count(&self) -> u32 {
        self.player_count
    }

    pub fn current_player(&self) -> TurnId {
        self.current_player
    }

    pub fn next_turn(&self) -> TurnManager {
        TurnManager {
            player_count: self.player_count,
            current_player: TurnId((self.current_player.0 + 1) % self.player_count),
        }
    }
}
