use super::*;
use std::collections::HashSet;

impl Model {
    pub fn tick(&mut self) {
        for tile in self.tiles.values_mut() {
            tile.updated = false;
        }

        let time = Instant::now();
        let (moves, cant_move) = self.calc_movement();
        println!("calculate: {}", time.elapsed().as_millis());

        let time = Instant::now();
        self.movement(&moves);
        println!("movement: {}", time.elapsed().as_millis());

        let time = Instant::now();
        for pos in &cant_move {
            let tile = self.tiles.get_mut(pos).unwrap();
            tile.needs_update = false;
        }

        for (&move_from, &move_to) in &moves {
            self.update_view
                .update_tile(move_from, self.tiles.get(&move_from).cloned());
            self.update_view
                .update_tile(move_to, self.tiles.get(&move_to).cloned());
        }
        println!("update tiles: {}", time.elapsed().as_millis());
    }

    fn calc_movement(&mut self) -> (HashMap<IVec2, IVec2>, HashSet<IVec2>) {
        let mut checked = HashSet::new();
        let mut moves = HashMap::new();
        let mut moves_to = HashSet::new();
        let mut cant_move = HashSet::new();
        let mut update_tiles: HashSet<IVec2> = self
            .tiles
            .iter()
            .filter_map(|(&pos, tile)| if tile.needs_update { Some(pos) } else { None })
            .collect();

        while let Some(&pos) = update_tiles.iter().next() {
            self.can_move(
                pos,
                &mut checked,
                &mut moves,
                &mut moves_to,
                &mut cant_move,
                &mut update_tiles,
            );
        }

        (moves, cant_move)
    }

    fn can_move(
        &self,
        position: IVec2,
        checked: &mut HashSet<IVec2>,
        moves: &mut HashMap<IVec2, IVec2>,
        moves_to: &mut HashSet<IVec2>,
        cant_move: &mut HashSet<IVec2>,
        update_tiles: &mut HashSet<IVec2>,
    ) -> bool {
        if let Some(tile) = self.tiles.get(&position) {
            update_tiles.remove(&position);
            if moves.contains_key(&position) {
                return true;
            }
            if !checked.insert(position) {
                return false;
            };

            let directions = tile.move_directions();
            for direction in directions {
                let target_pos = position + direction;
                if !moves_to.contains(&target_pos)
                    && self.can_move(
                        target_pos,
                        checked,
                        moves,
                        moves_to,
                        cant_move,
                        update_tiles,
                    )
                {
                    moves.insert(position, target_pos);
                    moves_to.insert(target_pos);
                    self.add_update_tiles_around(update_tiles, position, 1, moves, cant_move);
                    return true;
                }
            }

            cant_move.insert(position);
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
        let tile = self.tiles.get(&move_from).unwrap();
        if tile.updated {
            return;
        }

        if self.tiles.contains_key(&move_to) {
            let &next_move = moves.get(&move_to).unwrap();
            self.move_tile(move_to, next_move, moves);
        }
        let mut tile = self.tiles.remove(&move_from).unwrap();
        tile.position = move_to;
        tile.updated = true;
        if let Some(tile) = self.tiles.insert(move_to, tile) {
            panic!("Trying to move tile into another tile {:?}", tile);
        }
    }

    fn add_update_tiles_around(
        &self,
        update_tiles: &mut HashSet<IVec2>,
        position: IVec2,
        square_radius: i32,
        moves: &HashMap<IVec2, IVec2>,
        cant_move: &HashSet<IVec2>,
    ) {
        for dx in -square_radius..=square_radius {
            for dy in -square_radius..=square_radius {
                if dx != 0 || dy != 0 {
                    let pos = position + ivec2(dx, dy);
                    if self.tiles.contains_key(&pos)
                        && !moves.contains_key(&pos)
                        && !cant_move.contains(&pos)
                    {
                        update_tiles.insert(pos);
                    }
                }
            }
        }
    }
}
