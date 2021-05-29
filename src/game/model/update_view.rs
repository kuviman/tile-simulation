use super::*;

#[derive(Default)]
pub struct UpdateView {
    pub tiles: HashMap<IVec2, Option<Tile>>,
}

impl UpdateView {
    pub fn update_view<'a>(&mut self, tiles: impl Iterator<Item = (&'a IVec2, Option<&'a Tile>)>) {
        self.tiles
            .extend(tiles.map(|(&pos, tile)| (pos, tile.cloned())));
    }
    pub fn update_tile(&mut self, tile_pos: IVec2, tile: Option<Tile>) {
        self.tiles.insert(tile_pos, tile);
    }
}
