use macroquad::prelude::{ivec2, uvec2, IVec2, UVec2};
use std::collections::HashMap;

use crate::constants::{CHUNK_SIZE, CHUNK_SIZE_X, CHUNK_SIZE_Y};

pub type DataArray<T> = [T; CHUNK_SIZE];

pub fn data_array<T: Copy>(default_value: T) -> DataArray<T> {
    [default_value; CHUNK_SIZE]
}

pub fn tile_index_to_position(tile_index: usize) -> IVec2 {
    let y = tile_index / CHUNK_SIZE_X;
    assert!(y < CHUNK_SIZE_Y);
    ivec2(tile_index as i32 % CHUNK_SIZE_X as i32, y as i32)
}

pub fn tile_position_to_index(tile_position: UVec2) -> usize {
    assert!(
        tile_position.x < CHUNK_SIZE_X as u32 && tile_position.y < CHUNK_SIZE_Y as u32,
        "position {} out of chunk bounds",
        tile_position
    );
    let index = tile_position.x as usize + tile_position.y as usize * CHUNK_SIZE_X;
    index
}

pub struct Chunk {
    tiles: DataArray<bool>,
    need_update: DataArray<bool>,
}

impl Chunk {
    pub fn empty() -> Self {
        Self {
            tiles: data_array(false),
            need_update: data_array(false),
        }
    }

    pub fn tiles(&self) -> impl Iterator<Item = (usize, &bool)> {
        self.tiles.iter().enumerate()
    }

    pub fn set_tile(&mut self, index: usize) {
        self.tiles[index] = true;
        self.need_update[index] = true;
    }

    pub fn start_calculation(&mut self) -> MovementCalculation {
        let mut calculation = MovementCalculation {
            checked: data_array(false),
            moves: HashMap::new(),
            moves_to: data_array(false),
            cant_move: data_array(false),
            update_tiles: {
                let mut update_tiles = Vec::new();
                for index in 0..self.need_update.len() {
                    if self.need_update[index] {
                        if self.tiles[index] {
                            update_tiles.push(index);
                        } else {
                            self.need_update[index] = false;
                        }
                    }
                }
                update_tiles
            },
            unknown: data_array(false),
            extra_updates: Vec::new(),
            view_update: data_array(None),
        };

        while !calculation.update_tiles.is_empty() {
            let update_index = calculation.update_tiles.remove(0);
            self.update_move(update_index, &mut calculation);
        }

        calculation
    }

    fn update_move(
        &mut self,
        update_index: usize,
        calculation: &mut MovementCalculation,
    ) -> MoveInfo {
        // If there is no tile
        // or we've calculated that this tile can move,
        // then movement is allowed
        if !self.tiles[update_index] || calculation.moves.contains_key(&update_index) {
            return MoveInfo::Possible;
        }

        // if this tile's behaviour is unknown,
        // then movement is unknown
        if calculation.unknown[update_index] {
            return MoveInfo::Unknown;
        }

        // If we've alredy checked this tile
        // or another tile is going to move here
        // or this tile's behaviour is unknown,
        // then movement is not allowed
        let checked = calculation.checked[update_index];
        calculation.checked[update_index] = true;
        if checked || calculation.moves_to[update_index] {
            return MoveInfo::Impossible;
        }

        // Check for possible moves
        let directions = vec![ivec2(0, -1), ivec2(-1, -1), ivec2(1, -1)];
        for direction in directions {
            // Check if target is inside the current chunk
            match Self::shift_position(update_index, direction) {
                Ok(target_index) => {
                    // Inside the current chunk -> check if movement is possible
                    match self.update_move(target_index, calculation) {
                        MoveInfo::Impossible => (),
                        MoveInfo::Possible => {
                            // Register the move
                            calculation.moves.insert(update_index, target_index);
                            calculation.moves_to[target_index] = true;

                            // Update view
                            calculation.view_update[target_index] = Some(true);
                            if !calculation.moves_to[update_index] {
                                calculation.view_update[update_index] = Some(false);
                            }

                            // Queue update for the next frame
                            self.need_update[target_index] = true;

                            // Update nearby lazy tiles
                            self.update_tiles_around(update_index, 1, calculation);
                            return MoveInfo::Possible;
                        }
                        MoveInfo::Unknown => {
                            calculation.unknown[update_index] = true;
                            return MoveInfo::Unknown;
                        }
                    }
                }
                Err((chunk_shift, tile_index)) => {
                    // Outside of the current chunk -> behaviour is unknown
                    calculation.unknown[update_index] = true;
                    return MoveInfo::Unknown;
                }
            }
        }

        // There are no possible moves
        calculation.cant_move[update_index] = true;
        // Set this tile into lazy mode
        self.need_update[update_index] = false;
        MoveInfo::Impossible
    }

    fn update_tiles_around(
        &self,
        index: usize,
        distance: i32,
        calculation: &mut MovementCalculation,
    ) {
        // Update tiles in a square around a given tile
        for dx in -distance..=distance {
            for dy in -distance..=distance {
                let shift = ivec2(dx, dy);
                match Self::shift_position(index, shift) {
                    Ok(index) => {
                        // Tile is inside the chunk -> queue update
                        if self.tiles[index]
                            && !self.need_update[index]
                            && !calculation.checked[index]
                        {
                            println!("Updating lazy tile");
                            calculation.update_tiles.push(index);
                        }
                    }
                    Err((chunk_shift, index)) => {
                        // Tile is outside the chunk -> queue global update
                        calculation.extra_updates.push((chunk_shift, index));
                    }
                }
            }
        }
    }

    fn shift_position(tile_index: usize, shift: IVec2) -> Result<usize, (IVec2, usize)> {
        // Translate tile index into a vector
        let position = tile_index_to_position(tile_index) + shift;

        // Check if new position is outside the chunk
        let (chunk_shift_x, tile_pos_x) = if position.x < 0 {
            (-1, (position.x + CHUNK_SIZE_X as i32) as u32)
        } else if position.x >= CHUNK_SIZE_X as i32 {
            (1, (position.x - CHUNK_SIZE_X as i32) as u32)
        } else {
            (0, position.x as u32)
        };

        let (chunk_shift_y, tile_pos_y) = if position.y < 0 {
            (-1, (position.y + CHUNK_SIZE_Y as i32) as u32)
        } else if position.y >= CHUNK_SIZE_Y as i32 {
            (1, (position.y - CHUNK_SIZE_Y as i32) as u32)
        } else {
            (0, position.y as u32)
        };

        let tile_position = uvec2(tile_pos_x, tile_pos_y);

        if chunk_shift_x == 0 && chunk_shift_y == 0 {
            // Inside the chunk
            Ok(tile_position_to_index(tile_position))
        } else {
            // Outside the chunk
            Err((
                ivec2(chunk_shift_x, chunk_shift_y),
                tile_position_to_index(tile_position),
            ))
        }
    }

    pub fn start_movement(&mut self, moves: HashMap<usize, usize>) {
        let mut updated = data_array(false);
        for (&move_from, &move_to) in &moves {
            self.move_tile(move_from, move_to, &moves, &mut updated);
        }
    }

    fn move_tile(
        &mut self,
        move_from: usize,
        move_to: usize,
        moves: &HashMap<usize, usize>,
        updated: &mut DataArray<bool>,
    ) {
        if updated[move_from] {
            return;
        }

        if self.tiles[move_to] {
            let next_move = moves[&move_to];
            self.move_tile(move_to, next_move, &moves, updated);
        }

        assert!(
            !self.tiles[move_to],
            "Trying to move a tile into another tile!"
        );

        self.tiles[move_from] = false;
        updated[move_from] = true;
        self.tiles[move_to] = true;
    }
}

pub struct MovementCalculation {
    pub checked: DataArray<bool>,
    pub moves: HashMap<usize, usize>,
    pub moves_to: DataArray<bool>,
    pub cant_move: DataArray<bool>,
    pub update_tiles: Vec<usize>,
    pub unknown: DataArray<bool>,
    pub extra_updates: Vec<(IVec2, usize)>,
    pub view_update: DataArray<Option<bool>>,
}

pub enum MoveInfo {
    Impossible,
    Possible,
    Unknown,
}
