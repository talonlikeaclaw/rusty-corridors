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

    // Renders the Player on the Map
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
