use bracket_lib::prelude::*;
use prelude::*;

mod map;
mod map_builder;
mod player;

// Defines the crate's prelude
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

// Represents the State of the game
struct State {
    map: Map,
    player: Player,
}

impl State {
    // Constructor for the State
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    // Defines the required tick function from GameState
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        self.player.render(ctx);
        self.player.update(ctx, &self.map);
    }
}

fn main() -> BError {
    // Sets up the context
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Corridors")
        .with_fps_cap(30.0)
        .build()?;

    // Runs the main loop via context and State
    main_loop(context, State::new())
}
