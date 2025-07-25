use crate::prelude::*;
const NUM_ROOMS: usize = 20;

// Represents a map builder
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    // Fills in all tiles on the map with a tile
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}
