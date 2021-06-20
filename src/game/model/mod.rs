use super::*;

mod chunk;
mod tick;
mod tile;
mod update_view;

use chunk::*;
pub use tile::*;
pub use update_view::*;

pub struct Model {
    chunk_size: IVec2,
    chunks: HashMap<IVec2, Chunk>,
    update_view: UpdateView,
}

impl Model {
    pub fn new() -> Self {
        let chunk_size = ivec2(25, 25);
        let mut model = Self {
            chunks: {
                let mut chunks = HashMap::new();
                for x in -5..=5 {
                    for y in -5..=5 {
                        let pos = ivec2(x, y);
                        chunks.insert(pos, Chunk::empty(pos, chunk_size));
                    }
                }
                chunks
            },
            // tiles: {
            //     let mut tiles = HashMap::new();
            //     for x in -100..100 {
            //         let position = ivec2(x, 0);
            //         tiles.insert(
            //             position,
            //             Tile {
            //                 updated: false,
            //                 needs_update: true,
            //                 position,
            //                 content: TileContent::Solid {
            //                     tile_solid: TileSolid::Barrier,
            //                 },
            //             },
            //         );
            //     }
            //     for y in 0..50 {
            //         let position = ivec2(-100, y);
            //         tiles.insert(
            //             position,
            //             Tile {
            //                 updated: false,
            //                 needs_update: true,
            //                 position,
            //                 content: TileContent::Solid {
            //                     tile_solid: TileSolid::Barrier,
            //                 },
            //             },
            //         );
            //         let position = ivec2(99, y);
            //         tiles.insert(
            //             position,
            //             Tile {
            //                 updated: false,
            //                 needs_update: true,
            //                 position,
            //                 content: TileContent::Solid {
            //                     tile_solid: TileSolid::Barrier,
            //                 },
            //             },
            //         );
            //     }
            //     tiles
            // },
            update_view: UpdateView::default(),
            chunk_size,
        };
        model.update_view.update_view(
            model
                .chunks
                .values()
                .map(|chunk| chunk.tiles().map(|(pos, tile)| (pos, Some(tile))))
                .flatten(),
        );
        model
    }

    pub fn change_tile(&mut self, tile_pos: IVec2, tile: Option<Tile>) {
        self.update_view.update_tile(tile_pos, tile.clone());
        match tile {
            Some(tile) => {
                self.set_tile(
                    tile_pos,
                    Tile {
                        position: tile_pos,
                        ..tile
                    },
                );
            }
            None => {
                self.remove_tile(tile_pos);
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
                if let Some(tile) = self.get_tile_mut(position) {
                    tile.needs_update = true;
                }
            }
        }
    }

    fn get_tile(&self, tile_pos: IVec2) -> Option<&Tile> {
        self.get_tile_chunk(tile_pos)
            .and_then(|chunk| chunk.get_tile(tile_pos))
    }

    fn get_tile_mut(&mut self, tile_pos: IVec2) -> Option<&mut Tile> {
        self.get_tile_chunk_mut(tile_pos)
            .and_then(|chunk| chunk.get_tile_mut(tile_pos))
    }

    fn set_tile(&mut self, tile_pos: IVec2, tile: Tile) -> Option<Tile> {
        match self.get_tile_chunk_mut(tile_pos) {
            Some(chunk) => chunk.set_tile(tile_pos, tile),
            None => Some(tile),
        }
    }

    fn remove_tile(&mut self, tile_pos: IVec2) -> Option<Tile> {
        self.get_tile_chunk_mut(tile_pos)
            .and_then(|chunk| chunk.remove_tile(tile_pos))
    }

    fn get_tile_chunk(&self, tile_pos: IVec2) -> Option<&Chunk> {
        let chunk_pos = self.get_tile_chunk_pos(tile_pos);
        self.chunks.get(&chunk_pos)
    }

    fn get_tile_chunk_mut(&mut self, tile_pos: IVec2) -> Option<&mut Chunk> {
        let chunk_pos = self.get_tile_chunk_pos(tile_pos);
        self.chunks.get_mut(&chunk_pos)
    }

    fn get_tile_chunk_pos(&self, tile_pos: IVec2) -> IVec2 {
        ivec2(
            tile_pos.x / self.chunk_size.x,
            tile_pos.y / self.chunk_size.y,
        )
    }
}
