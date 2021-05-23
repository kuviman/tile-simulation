use macroquad::prelude::{mouse_position, MouseButton};

use super::*;

impl Game {
    pub fn update(&mut self, delta_time: f32) {
        self.current_fps = 1.0 / delta_time;

        if is_mouse_button_down(MouseButton::Left) {
            let tile_pos = self.mouse_tile_pos();
            self.tiles.insert(
                tile_pos,
                Tile {
                    updated: false,
                    position: tile_pos,
                    content: TileContent::Solid {
                        tile_solid: TileSolid::Sand,
                    },
                },
            );
        }
    }

    fn mouse_tile_pos(&self) -> IVec2 {
        let mouse_pos = mouse_position();
        let mouse_pos = vec2(mouse_pos.0, mouse_pos.1);
        let world_pos = self.game_camera.screen_to_world(mouse_pos);
        ivec2(world_pos.x.floor() as i32, world_pos.y.floor() as i32)
    }
}
