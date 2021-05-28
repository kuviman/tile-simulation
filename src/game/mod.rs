use super::*;
use std::collections::HashMap;

mod model;
mod renderer;

use model::*;
use renderer::*;

pub struct Game {
    renderer: Renderer,
    model: Model,
    selected_tile: Option<Tile>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            model: Model::new(),
            selected_tile: None,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.renderer.update(delta_time);

        if is_mouse_button_down(MouseButton::Left) {
            let tile_pos = self.mouse_tile_pos();
            match &self.selected_tile {
                Some(tile) => {
                    self.model.tiles.insert(
                        tile_pos,
                        Tile {
                            position: tile_pos,
                            ..tile.clone()
                        },
                    );
                }
                None => {
                    self.model.tiles.remove(&tile_pos);
                }
            }
        }

        if let Some(tile_content) = if is_key_down(KeyCode::Key1) {
            self.selected_tile = None;
            None
        } else if is_key_down(KeyCode::Key2) {
            Some(TileContent::Solid {
                tile_solid: TileSolid::Barrier,
            })
        } else if is_key_down(KeyCode::Key3) {
            Some(TileContent::Solid {
                tile_solid: TileSolid::Sand,
            })
        } else if is_key_down(KeyCode::Key4) {
            Some(TileContent::Liquid {
                tile_liquid: TileLiquid::Water,
            })
        } else if is_key_down(KeyCode::Key5) {
            Some(TileContent::Gas {
                tile_gas: TileGas::Smoke,
            })
        } else {
            None
        } {
            self.selected_tile = Some(Tile {
                updated: false,
                needs_update: true,
                position: IVec2::ZERO,
                content: tile_content,
            })
        }
    }

    pub fn fixed_update(&mut self, _delta_time: f32) {
        self.model.tick();
    }

    pub fn draw(&self) {
        self.renderer.draw(&self.model);
    }

    fn mouse_tile_pos(&self) -> IVec2 {
        let mouse_pos = mouse_position();
        let mouse_pos = vec2(mouse_pos.0, mouse_pos.1);
        let world_pos = self.renderer.game_camera.screen_to_world(mouse_pos);
        ivec2(world_pos.x.floor() as i32, world_pos.y.floor() as i32)
    }
}
