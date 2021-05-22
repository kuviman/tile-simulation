use macroquad::prelude::*;
use std::collections::HashMap;

mod draw;
mod fixed_update;
mod tile;
mod update;

use tile::*;

pub struct Game {
    game_camera: Camera2D,
    tiles: HashMap<IVec2, Tile>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_camera: Camera2D {
                offset: vec2(0.0, -0.75),
                zoom: vec2(0.01, 0.01 * screen_width() / screen_height()),
                ..Default::default()
            },
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
