use std::collections::HashMap;

use super::*;

#[derive(Default)]
pub struct UpdateView {
    tiles: HashMap<IVec2, bool>,
}

impl UpdateView {
    pub fn tiles(self) -> impl Iterator<Item = (IVec2, bool)> {
        self.tiles.into_iter()
    }

    pub fn update_view(&mut self, tiles: impl Iterator<Item = (IVec2, bool)>) {
        self.tiles.extend(tiles);
    }

    pub fn update_tile(&mut self, tile_pos: IVec2, tile: bool) {
        self.tiles.insert(tile_pos, tile);
    }
}
