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
                for x in -50..=50 {
                    let position = ivec2(x, 0);
                    tiles.insert(
                        position,
                        Tile {
                            updated: false,
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
