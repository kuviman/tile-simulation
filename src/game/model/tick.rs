use super::*;
use std::collections::HashSet;

impl Model {
    pub fn tick(&mut self) {
        for tile in self.tiles.values_mut() {
            tile.updated = false;
        }

        let (moves, cant_move) = self.calc_movement();

        for pos in &cant_move {
            let tile = self.tiles.get_mut(pos).unwrap();
            tile.needs_update = false;
        }

        self.movement(&moves);
    }

    fn calc_movement(&mut self) -> (HashMap<IVec2, IVec2>, HashSet<IVec2>) {
        let mut moves = HashMap::new();
        let mut cant_move = HashSet::new();
        let mut update_tiles: HashSet<IVec2> = self
            .tiles
            .iter()
            .filter_map(|(&pos, tile)| if tile.needs_update { Some(pos) } else { None })
            .collect();

        while !update_tiles.is_empty() {
            let mut next_update = HashSet::new();
            for &pos in &update_tiles {
                self.can_move(
                    pos,
                    &mut HashSet::new(),
                    &mut moves,
                    &mut cant_move,
                    &mut next_update,
                );
            }

            next_update.retain(|pos| {
                if let Some(tile) = self.tiles.get_mut(pos) {
                    tile.needs_update = true;
                    true
                } else {
                    false
                }
            });
            for update_pos in &next_update {
                cant_move.remove(update_pos);
            }
            update_tiles = next_update;
        }

        (moves, cant_move)
    }

    fn can_move(
        &self,
        position: IVec2,
        check: &mut HashSet<IVec2>,
        moves: &mut HashMap<IVec2, IVec2>,
        cant_move: &mut HashSet<IVec2>,
        update_tiles: &mut HashSet<IVec2>,
    ) -> bool {
        if moves.contains_key(&position) {
            return true;
        }
        if !check.insert(position) || cant_move.contains(&position) {
            return false;
        }

        if let Some(tile) = self.tiles.get(&position) {
            if tile.needs_update {
                let directions = tile.move_directions();
                for direction in directions {
                    let target_pos = position + direction;
                    if self.can_move(target_pos, check, moves, cant_move, update_tiles)
                        && !moves.values().any(|&move_to| move_to == target_pos)
                    {
                        moves.insert(position, target_pos);
                        self.add_update_tiles_around(update_tiles, position, 1);
                        return true;
                    }
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
    ) {
        for dx in -square_radius..=square_radius {
            for dy in -square_radius..=square_radius {
                let pos = position + ivec2(dx, dy);
                update_tiles.insert(pos);
            }
        }
    }
}
