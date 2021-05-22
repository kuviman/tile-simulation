use super::*;

impl Game {
    pub fn fixed_update(&mut self, delta_time: f32) {
        for (_, tile) in &mut self.tiles {
            tile.updated = false;
        }
        let moves = self.calc_movement();
        self.movement(moves);
    }

    fn calc_movement(&mut self) -> HashMap<IVec2, IVec2> {
        let mut moves = HashMap::new();
        for (&pos, tile) in &self.tiles {
            if tile.updated {
                continue;
            }

            let directions = tile.move_directions();
            for direction in directions {
                let target_pos = pos + direction;
                if !self.tiles.contains_key(&target_pos) {
                    moves.insert(pos, target_pos);
                    break;
                }
            }
        }
        moves
    }

    fn movement(&mut self, moves: HashMap<IVec2, IVec2>) {
        for (&move_from, &move_to) in &moves {
            self.move_tile(move_from, move_to, &moves);
        }
    }

    fn move_tile(&mut self, move_from: IVec2, move_to: IVec2, moves: &HashMap<IVec2, IVec2>) {
        let tile = self.tiles.get(&move_from).unwrap();
        if tile.updated {
            return;
        }

        if let Some(_) = self.tiles.get(&move_to) {
            if let Some(&next_move) = moves.get(&move_to) {
                self.move_tile(move_to, next_move, moves);
            } else {
                return;
            }
        }
        let mut tile = self.tiles.remove(&move_from).unwrap();
        tile.position = move_to;
        tile.updated = true;
        if self.tiles.insert(move_to, tile).is_some() {
            panic!("Trying to move tile into another tile");
        }
    }
}
