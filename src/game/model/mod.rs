use super::*;

mod tick;
mod tile;

pub use tile::*;

pub struct Model {
    pub tiles: HashMap<IVec2, Tile>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            tiles: {
                let mut tiles = HashMap::new();
                for x in -100..100 {
                    let position = ivec2(x, 0);
                    tiles.insert(
                        position,
                        Tile {
                            updated: false,
                            needs_update: true,
                            position,
                            content: TileContent::Solid {
                                tile_solid: TileSolid::Barrier,
                            },
                        },
                    );
                }
                for y in 0..50 {
                    let position = ivec2(-100, y);
                    tiles.insert(
                        position,
                        Tile {
                            updated: false,
                            needs_update: true,
                            position,
                            content: TileContent::Solid {
                                tile_solid: TileSolid::Barrier,
                            },
                        },
                    );
                    let position = ivec2(99, y);
                    tiles.insert(
                        position,
                        Tile {
                            updated: false,
                            needs_update: true,
                            position,
                            content: TileContent::Solid {
                                tile_solid: TileSolid::Barrier,
                            },
                        },
                    );
                }
                tiles
            },
        }
    }
}
