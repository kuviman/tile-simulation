use macroquad::prelude::{ivec2, IVec2};
use std::{collections::HashMap, sync::Mutex};

use crate::{
    constants::{CHUNK_SIZE_X, CHUNK_SIZE_Y},
    game::calculator::Calculator,
};

use super::{
    chunk::{tile_index_to_position, DataArray},
    tile::Tile,
    Game,
};

impl Game {
    pub fn tick(&mut self) {
        // Calculate movement
        let calculation = self.calculate_movement();

        // // Update view
        // for (chunk_pos, update_view) in calculation.view_update {
        //     for (index, update) in update_view
        //         .iter()
        //         .enumerate()
        //         .filter_map(|(index, update)| update.map(|update| (index, update)))
        //     {
        //         let tile_pos = tile_index_to_position(index)
        //             + chunk_pos * ivec2(CHUNK_SIZE_X as i32, CHUNK_SIZE_Y as i32);
        //         self.update_view.update_tile(tile_pos, update);
        //     }
        // }

        // // Perform movement
        // self.movement(calculation.chunk_moves, calculation.cross_moves);
    }

    fn calculate_movement(&mut self) -> Calculation {
        // Calculate chunks mostly in parallel (except for cross-chunk moves and updates)
        use rayon::prelude::*;
        let calculator = Mutex::new(Calculator::new(self.chunks.len()));

        let calculations = self
            .chunks
            .iter_mut()
            .map(|(&chunk_pos, chunk)| (chunk_pos, chunk.calculate(&calculator)))
            .collect::<Vec<_>>();

        println!("All good");

        // let mut view_update = HashMap::with_capacity(calculations.len());
        // let mut chunk_moves = HashMap::with_capacity(calculations.len());
        // let mut cross_moves = HashMap::new();

        for (chunk_pos, calculation) in calculations {
            // view_update.insert(chunk_pos, calculation.view_update);
            // chunk_moves.insert(chunk_pos, calculation.moves);
            // cross_moves.insert(chunk_pos, calculation.cross_moves);
        }

        Calculation {
            view_update: HashMap::new(),
            chunk_moves: HashMap::new(),
            cross_moves: HashMap::new(),
        }
    }

    fn movement(
        &mut self,
        chunk_moves: HashMap<IVec2, DataArray<Option<usize>>>,
        mut cross_moves: HashMap<IVec2, DataArray<Option<Tile>>>,
    ) {
        // // Perform simple moves
        // for (chunk_pos, moves) in chunk_moves {
        //     self.chunks
        //         .get_mut(&chunk_pos)
        //         .unwrap()
        //         .start_movement(moves);
        // }

        // // Perform cross-chunk moves
        // while let Some(&move_from) = cross_moves.keys().next() {
        //     self.move_tile(move_from, &mut cross_moves);
        // }
    }

    fn move_tile(&mut self, move_from: Tile, moves: &mut HashMap<Tile, Tile>) {
        let move_to = moves.remove(&move_from).unwrap();

        if moves.contains_key(&move_to) {
            self.move_tile(move_to, moves);
        }

        self.chunks.get_mut(&move_from.chunk_pos).unwrap().tiles[move_from.index] = false;
        self.chunks.get_mut(&move_to.chunk_pos).unwrap().tiles[move_to.index] = true;
    }
}

struct Calculation {
    view_update: HashMap<IVec2, DataArray<Option<bool>>>,
    chunk_moves: HashMap<IVec2, DataArray<Option<usize>>>,
    cross_moves: HashMap<IVec2, DataArray<Option<Tile>>>,
}

// fn update_calculations(
//     mut unknowns: Vec<Tile>,
//     chunk_moves: HashMap<IVec2, DataArray<Option<usize>>>,
//     mut calculations: HashMap<IVec2, (&mut Chunk, UpdatedCalculation)>,
// ) -> Calculation {
//     let mut cross_moves = HashMap::new();
//     while !unknowns.is_empty() {
//         let unknown_tile = unknowns.remove(0);
//         update_move(
//             unknown_tile,
//             &mut unknowns,
//             &mut cross_moves,
//             &mut calculations,
//         );
//     }

//     let mut update_view = HashMap::with_capacity(calculations.len());
//     for (chunk_pos, (_, calculation)) in calculations {
//         update_view.insert(chunk_pos, calculation.view_update);
//     }

//     Calculation {
//         update_view,
//         chunk_moves,
//         cross_moves,
//     }
// }

// fn update_move(
//     update_tile: Tile,
//     unknowns: &mut Vec<Tile>,
//     moves: &mut HashMap<Tile, Tile>,
//     calculations: &mut HashMap<IVec2, (&mut Chunk, UpdatedCalculation)>,
// ) -> bool {
//     let (chunk, calculation) = match calculations.get_mut(&update_tile.chunk_pos) {
//         Some(result) => result,
//         None => {
//             return false;
//         }
//     };

//     // If there is no tile
//     // or we've calculated that this tile can move,
//     // then movement is allowed
//     if !chunk.tiles[update_tile.index]
//         || calculation.moves[update_tile.index].is_some()
//         || moves.contains_key(&update_tile)
//     {
//         return true;
//     }

//     // If we've alredy checked this tile
//     // or another tile is going to move here
//     // or this tile's behaviour is unknown,
//     // then movement is not allowed
//     let checked = calculation.checked[update_tile.index];
//     calculation.checked[update_tile.index] = true;
//     if checked || calculation.moves_to[update_tile.index] {
//         return false;
//     }

//     // Check for possible moves
//     let directions = vec![ivec2(0, -1), ivec2(-1, -1), ivec2(1, -1)];
//     for direction in directions {
//         // Find target tile
//         let move_to = match Chunk::shift_position(update_tile.index, direction) {
//             Ok(index) => Tile {
//                 chunk_pos: update_tile.chunk_pos,
//                 index,
//             },
//             Err((chunk_shift, index)) => Tile {
//                 chunk_pos: update_tile.chunk_pos + chunk_shift,
//                 index,
//             },
//         };

//         // Check if move is possible
//         if update_move(move_to, unknowns, moves, calculations) {
//             // Update chunk from
//             let (_, calculation) = calculations.get_mut(&update_tile.chunk_pos).unwrap();

//             // Register the move
//             moves.insert(update_tile, move_to);
//             if update_tile.chunk_pos == move_to.chunk_pos {
//                 calculation.moves[update_tile.index] = Some(move_to.index);
//             }

//             // Update view
//             if !calculation.moves_to[update_tile.index] {
//                 calculation.view_update[update_tile.index] = Some(false);
//             }

//             // Update chunk to
//             let (chunk, calculation) = calculations.get_mut(&move_to.chunk_pos).unwrap();

//             // Register the move
//             calculation.moves_to[move_to.index] = true;

//             // Update view
//             calculation.view_update[move_to.index] = Some(true);

//             // Queue update for the next frame
//             chunk.need_update[move_to.index] = true;

//             // Update nearby lazy tiles
//             update_tiles_around(update_tile, 1, unknowns, calculations);
//             return true;
//         }
//     }

//     // There are no possible moves
//     let (chunk, calculation) = calculations.get_mut(&update_tile.chunk_pos).unwrap();
//     calculation.cant_move[update_tile.index] = true;
//     // Set this tile into lazy mode
//     chunk.need_update[update_tile.index] = false;
//     false
// }

// fn update_tiles_around(
//     tile: Tile,
//     distance: i32,
//     unknowns: &mut Vec<Tile>,
//     calculations: &HashMap<IVec2, (&mut Chunk, UpdatedCalculation)>,
// ) {
//     // Update tiles in a square around a given tile
//     for dx in -distance..=distance {
//         for dy in -distance..=distance {
//             let shift = ivec2(dx, dy);
//             // Find the tile to update
//             let update_tile = match Chunk::shift_position(tile.index, shift) {
//                 Ok(index) => Tile {
//                     chunk_pos: tile.chunk_pos,
//                     index,
//                 },
//                 Err((chunk_shift, index)) => Tile {
//                     chunk_pos: tile.chunk_pos + chunk_shift,
//                     index,
//                 },
//             };

//             // Check if tile needs updating
//             if let Some((chunk, calculation)) = calculations.get(&update_tile.chunk_pos) {
//                 if chunk.tiles[update_tile.index]
//                     && !chunk.need_update[update_tile.index]
//                     && !calculation.checked[update_tile.index]
//                 {
//                     unknowns.push(update_tile);
//                 }
//             }
//         }
//     }
// }
