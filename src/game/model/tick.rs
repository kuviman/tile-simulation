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
        for (&pos, tile) in &self.tiles {
            if !tile.needs_update {
                continue;
            }
            self.can_move(pos, &mut moves, &mut cant_move);
        }
        (moves, cant_move)
    }

    fn can_move(
        &self,
        position: IVec2,
        moves: &mut HashMap<IVec2, IVec2>,
        cant_move: &mut HashSet<IVec2>,
    ) -> bool {
        if moves.contains_key(&position) {
            return true;
        }
        if cant_move.contains(&position) {
            return false;
        }

        if let Some(tile) = self.tiles.get(&position) {
            if tile.needs_update {
                let directions = tile.move_directions();
                for direction in directions {
                    let target_pos = position + direction;
                    if self.can_move(target_pos, moves, cant_move)
                        && !moves.values().any(|&move_to| move_to == target_pos)
                    {
                        moves.insert(position, target_pos);
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
}
