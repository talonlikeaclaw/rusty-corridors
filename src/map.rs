use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// Represents the different types of tiles on the map
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

// Represents the dungeon map
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    // Constructor for maps
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // Checks that the location specified is within the bounds of the screen
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // Checks that a destination tile is valid (is on screen and is a floor tile)
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // Checks if an coordinate is valid and return option with index or none
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

// Calculates the tile index from x and y using row-first striding
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
