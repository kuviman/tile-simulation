use super::*;

mod tick;
mod tile;
mod update_view;

pub use tile::*;
pub use update_view::*;

pub struct Model {
    tiles: HashMap<IVec2, Tile>,
    update_view: UpdateView,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Self {
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
            update_view: UpdateView::default(),
        };
        model
            .update_view
            .update_view(model.tiles.iter().map(|(pos, tile)| (pos, Some(tile))));
        model
    }
    pub fn set_tile(&mut self, tile_pos: IVec2, tile: Option<Tile>) {
        self.update_view.update_tile(tile_pos, tile.clone());
        match tile {
            Some(tile) => {
                self.tiles.insert(
                    tile_pos,
                    Tile {
                        position: tile_pos,
                        ..tile
                    },
                );
            }
            None => {
                self.tiles.remove(&tile_pos);
            }
        }
        self.update_tiles(tile_pos, 1);
    }
    pub fn get_update_view(&mut self) -> UpdateView {
        std::mem::take(&mut self.update_view)
    }

    fn update_tiles(&mut self, position: IVec2, square_distance: i32) {
        for dx in -square_distance..=square_distance {
            for dy in -square_distance..=square_distance {
                let position = position + ivec2(dx, dy);
                if let Some(tile) = self.tiles.get_mut(&position) {
                    tile.needs_update = true;
                }
            }
        }
    }
}
