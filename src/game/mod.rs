use super::*;
use std::collections::HashMap;

mod model;
mod renderer;

use model::*;
use renderer::*;

pub struct Game {
    renderer: Renderer,
    model: Model,
}

impl Game {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            model: Model::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.renderer.update(delta_time);

        if is_mouse_button_down(MouseButton::Left) {
            let tile_pos = self.mouse_tile_pos();
            self.model.tiles.insert(
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

    pub fn fixed_update(&mut self, _delta_time: f32) {
        self.model.tick();
    }

    pub fn draw(&mut self) {
        self.renderer.draw(&self.model);
    }

    fn mouse_tile_pos(&self) -> IVec2 {
        let mouse_pos = mouse_position();
        let mouse_pos = vec2(mouse_pos.0, mouse_pos.1);
        let world_pos = self.renderer.game_camera.screen_to_world(mouse_pos);
        ivec2(world_pos.x.floor() as i32, world_pos.y.floor() as i32)
    }
}
