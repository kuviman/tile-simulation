use super::*;

impl Model {
    pub fn tick(&mut self) {
        let time = Instant::now();
        let calculations = self.calc_movement();
        let (moves, cant_move) = {
            let mut moves = HashMap::new();
            let mut cant_move = HashSet::new();
            for (chunk_pos, calculation) in calculations {
                let (calc_moves, calc_cant_move) = (calculation.moves, calculation.cant_move);
                moves.extend(calc_moves.into_iter().map(|(pos_from, pos_to)| {
                    let chunk = self.chunks.get(&chunk_pos).unwrap();
                    (chunk.get_tile_ivec2(pos_from), chunk.get_tile_ivec2(pos_to))
                }));
                cant_move.extend(calc_cant_move.into_iter().map(|pos| {
                    let chunk = self.chunks.get(&chunk_pos).unwrap();
                    chunk.get_tile_ivec2(pos)
                }));
            }
            (moves, cant_move)
        };
        println!("calculate: {}", time.elapsed().as_millis());

        let time = Instant::now();
        self.movement(&moves);
        println!("movement: {}", time.elapsed().as_millis());

        let time = Instant::now();
        for &pos in &cant_move {
            let tile = self.get_tile_mut(pos).unwrap();
            tile.needs_update = false;
        }

        for (&move_from, &move_to) in &moves {
            self.update_view
                .update_tile(move_from, self.get_tile(move_from).cloned());
            self.update_view
                .update_tile(move_to, self.get_tile(move_to).cloned());
        }
        println!("update tiles: {}", time.elapsed().as_millis());
    }

    fn calc_movement(&mut self) -> HashMap<IVec2, MovementCalculation> {
        let mut chunk_calculations = HashMap::with_capacity(self.chunks.len());
        for (&chunk_pos, chunk) in &mut self.chunks {
            let calculation = chunk.start_calculation();
            chunk_calculations.insert(chunk_pos, calculation);
        }
        for chunk in self.chunks.values() {
            self.update_calculation(chunk, &mut chunk_calculations);
        }
        chunk_calculations
    }

    pub fn update_calculation(
        &self,
        chunk: &Chunk,
        chunk_calculations: &mut HashMap<IVec2, MovementCalculation>,
    ) {
        while let Some(&update_pos) = chunk_calculations
            .get_mut(&chunk.chunk_pos)
            .unwrap()
            .update_tiles
            .iter()
            .next()
        {
            self.can_move(chunk, update_pos, chunk_calculations);
        }
    }

    fn can_move(
        &self,
        chunk: &Chunk,
        update_pos: Position,
        chunk_calculations: &mut HashMap<IVec2, MovementCalculation>,
    ) -> bool {
        if let Some(tile) = chunk.tiles.get(&update_pos) {
            let calculation = chunk_calculations.get_mut(&chunk.chunk_pos).unwrap();
            calculation.update_tiles.remove(&update_pos);
            if calculation.moves.contains_key(&update_pos) {
                return true;
            }
            if !calculation.checked.insert(update_pos)
                || calculation.moves_to.contains(&update_pos)
                || calculation.unknown.contains(&update_pos)
            {
                return false;
            };

            let directions = tile.move_directions();
            for direction in directions {
                match chunk.shift_position(update_pos, direction) {
                    Ok(target_pos) => {
                        if self.can_move(chunk, target_pos, chunk_calculations) {
                            let calculation = chunk_calculations.get_mut(&chunk.chunk_pos).unwrap();
                            calculation.moves.insert(update_pos, target_pos);
                            calculation.moves_to.insert(target_pos);
                            chunk.add_update_tiles_around(update_pos, 1, calculation);
                            return true;
                        }
                    }
                    Err((chunk_pos, tile_pos)) => {
                        if chunk_calculations.contains_key(&chunk_pos) {
                            let chunk = self.chunks.get(&chunk_pos).unwrap();
                            let target_pos = chunk.get_tile_position(tile_pos);
                            if self.can_move(chunk, target_pos, chunk_calculations) {
                                let calculation =
                                    chunk_calculations.get_mut(&chunk.chunk_pos).unwrap();
                                calculation.unknown.insert(update_pos);
                                return false;
                            }
                        }
                    }
                }
            }

            let calculation = chunk_calculations.get_mut(&chunk.chunk_pos).unwrap();
            calculation.cant_move.insert(update_pos);
            false
        } else {
            true
        }
    }

    fn movement(&mut self, moves: &HashMap<IVec2, IVec2>) {
        for (&move_from, &move_to) in moves {
            self.move_tile(move_from, move_to, &moves);
        }
    }

    fn move_tile(&mut self, move_from: IVec2, move_to: IVec2, moves: &HashMap<IVec2, IVec2>) {
        let tile = self.get_tile(move_from).unwrap();
        if tile.updated {
            return;
        }

        if self.get_tile(move_to).is_some() {
            let &next_move = moves.get(&move_to).unwrap();
            self.move_tile(move_to, next_move, moves);
        }
        let mut tile = self.remove_tile(move_from).unwrap();
        tile.position = move_to;
        tile.updated = true;
        if let Some(tile) = self.set_tile(move_to, tile) {
            panic!("Trying to move tile into another tile {:?}", tile);
        }
    }
}
