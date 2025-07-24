use bracket_lib::prelude::*;
use prelude::*;

mod map;

// Defines the crate's prelude
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

// Represents the State of the game
struct State {
    map: Map,
}

impl State {
    // Constructor for the State
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    // Defines the required tick function from GameState
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
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
