use crate::prelude::*;

// Represents a Player in the game
pub struct Player {
    pub position: Point,
}

impl Player {
    // Constuctor for the Player
    pub fn new(position: Point) -> Self {
        Self { position }
    }
}
