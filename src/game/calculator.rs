use macroquad::prelude::IVec2;
use std::collections::{HashMap, HashSet};

use super::{
    chunk::{data_array, DataArray, MoveInfo},
    tile::Tile,
};

pub struct Calculator {
    calculations: HashMap<IVec2, DataArray<MoveInfo>>,
    extra_updates: HashMap<IVec2, DataArray<bool>>,
    chunks: usize,
    updated: HashSet<IVec2>,
}

impl Calculator {
    pub fn new(chunks: usize) -> Self {
        Self {
            calculations: HashMap::new(),
            extra_updates: HashMap::new(),
            chunks,
            updated: HashSet::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.updated.len() == self.chunks
    }

    pub fn update(
        &mut self,
        chunk_pos: IVec2,
        chunk_updates: DataArray<Option<MoveInfo>>,
        extra_updates: Vec<Tile>,
        dependencies: &mut HashMap<Tile, MoveInfo>,
    ) -> Option<DataArray<bool>> {
        // Assume this chunk has not finished updating
        self.updated.remove(&chunk_pos);

        // Queue updates for other chunks
        for update_tile in extra_updates {
            let updates = self
                .extra_updates
                .entry(update_tile.chunk_pos)
                .or_insert_with(|| data_array(false));
            updates[update_tile.index] = true;
        }

        // Find or create chunk's calculation
        let tiles = self
            .calculations
            .entry(chunk_pos)
            .or_insert_with(|| data_array(MoveInfo::Unknown));

        // Update this chunk's calculation
        for (index, update) in chunk_updates
            .iter()
            .filter_map(|&update| update)
            .enumerate()
        {
            tiles[index] = update;
        }

        // Update dependencies
        let mut updated = false;
        for (tile, move_info) in dependencies {
            if let Some(calculation) = self.calculations.get(&tile.chunk_pos) {
                *move_info = calculation[tile.index];
                updated = true;
            }
        }

        // This chunk has finished updating
        if !updated {
            self.updated.insert(chunk_pos);
        }

        // Return updates from other chunks
        self.extra_updates.remove(&chunk_pos)
    }
}
