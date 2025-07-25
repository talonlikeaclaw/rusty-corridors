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

    // Updates the Player's delta depending on key input
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            // Determine attempted movement from keypress
            let delta = match key {
                VirtualKeyCode::H => Point::new(-1, 0),
                VirtualKeyCode::L => Point::new(1, 0),
                VirtualKeyCode::K => Point::new(0, -1),
                VirtualKeyCode::J => Point::new(0, 1),
                _ => Point::zero(), // No movement for other keys
            };

            let new_position = self.position + delta;

            // Update position only if player can enter the tile
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
