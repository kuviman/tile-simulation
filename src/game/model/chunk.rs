use super::*;

pub struct Chunk {
    pub chunk_pos: IVec2,
    chunk_size: IVec2,
    pub tiles: HashMap<Position, Tile>,
}

impl Chunk {
    pub fn empty(chunk_pos: IVec2, chunk_size: IVec2) -> Self {
        Self {
            chunk_pos,
            chunk_size,
            tiles: HashMap::new(),
        }
    }

    pub fn start_calculation(&mut self) -> MovementCalculation {
        MovementCalculation {
            checked: HashSet::new(),
            moves: HashMap::new(),
            moves_to: HashSet::new(),
            cant_move: HashSet::new(),
            update_tiles: self
                .tiles
                .iter_mut()
                .filter_map(|(&pos, tile)| {
                    tile.updated = false;
                    if tile.needs_update {
                        Some(pos)
                    } else {
                        None
                    }
                })
                .collect(),
            unknown: HashSet::new(),
            extra_updates: HashSet::new(),
        }
    }

    pub fn add_update_tiles_around(
        &self,
        position: Position,
        square_radius: i32,
        calculation: &mut MovementCalculation,
    ) {
        for dx in -square_radius..=square_radius {
            for dy in -square_radius..=square_radius {
                if dx != 0 || dy != 0 {
                    match self.shift_position(position, ivec2(dx, dy)) {
                        Ok(position) => {
                            if self.tiles.contains_key(&position)
                                && !calculation.moves.contains_key(&position)
                                && !calculation.cant_move.contains(&position)
                            {
                                calculation.update_tiles.insert(position);
                            }
                        }
                        Err((_, tile_pos)) => {
                            calculation.extra_updates.insert(tile_pos);
                        }
                    }
                }
            }
        }
    }

    pub fn shift_position(
        &self,
        position: Position,
        shift: IVec2,
    ) -> Result<Position, (IVec2, IVec2)> {
        let position = position.to_ivec2(self.chunk_size) + shift;
        let chunk_shift = ivec2(
            if position.x < 0 {
                -1
            } else if position.x >= self.chunk_size.x {
                1
            } else {
                0
            },
            if position.y < 0 {
                -1
            } else if position.y >= self.chunk_size.y {
                1
            } else {
                0
            },
        );

        if chunk_shift == IVec2::ZERO {
            Ok(Position::from_ivec2(position, self.chunk_size))
        } else {
            Err((
                self.chunk_pos + chunk_shift,
                position + self.chunk_pos * self.chunk_size,
            ))
        }
    }

    pub fn tiles(&self) -> impl Iterator<Item = (IVec2, &Tile)> {
        self.tiles
            .iter()
            .map(move |(&tile_pos, tile)| (self.get_tile_ivec2(tile_pos), tile))
    }

    pub fn get_tile(&self, tile_pos: IVec2) -> Option<&Tile> {
        let position = self.get_tile_position(tile_pos);
        self.tiles.get(&position)
    }

    pub fn get_tile_mut(&mut self, tile_pos: IVec2) -> Option<&mut Tile> {
        let position = self.get_tile_position(tile_pos);
        self.tiles.get_mut(&position)
    }

    pub fn set_tile(&mut self, tile_pos: IVec2, tile: Tile) -> Option<Tile> {
        let position = self.get_tile_position(tile_pos);
        self.tiles.insert(position, tile)
    }

    pub fn remove_tile(&mut self, tile_pos: IVec2) -> Option<Tile> {
        let position = self.get_tile_position(tile_pos);
        self.tiles.remove(&position)
    }

    pub fn get_tile_position(&self, tile_pos: IVec2) -> Position {
        let position =
            Position::from_ivec2(tile_pos - self.chunk_pos * self.chunk_size, self.chunk_size);
        position
    }

    pub fn get_tile_ivec2(&self, tile_pos: Position) -> IVec2 {
        tile_pos.to_ivec2(self.chunk_size) + self.chunk_pos * self.chunk_size
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    index: i32,
}

impl Position {
    pub fn to_ivec2(&self, chunk_size: IVec2) -> IVec2 {
        let y = self.index / chunk_size.x;
        assert!(y < chunk_size.y);
        ivec2(self.index % chunk_size.x, y)
    }
    pub fn from_ivec2(pos: IVec2, chunk_size: IVec2) -> Self {
        let position = Self {
            index: pos.x + pos.y * chunk_size.x,
        };
        assert!(
            position.index < chunk_size.x * chunk_size.y,
            "position ({} from {}) out of bounds",
            position.index,
            pos
        );
        position
    }
}

pub struct MovementCalculation {
    pub checked: HashSet<Position>,
    pub moves: HashMap<Position, Position>,
    pub moves_to: HashSet<Position>,
    pub cant_move: HashSet<Position>,
    pub update_tiles: HashSet<Position>,
    pub unknown: HashSet<Position>,
    pub extra_updates: HashSet<IVec2>,
}
