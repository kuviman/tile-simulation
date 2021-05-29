use super::*;

pub struct Renderer {
    pub game_camera: Camera2D,
    current_fps: f32,
    fps_update_time: f32,
    fps_update: f32,
    image: Image,
    texture: Texture2D,
}

impl Renderer {
    pub fn new() -> Self {
        let image = Image::gen_image_color(screen_width() as u16, screen_height() as u16, BLACK);
        Self {
            game_camera: Camera2D {
                offset: vec2(0.0, -1.0),
                zoom: vec2(0.01, 0.01 * screen_width() / screen_height()),
                ..Default::default()
            },
            current_fps: 0.0,
            fps_update_time: 0.5,
            fps_update: 0.0,
            texture: {
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Nearest);
                texture
            },
            image,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.fps_update -= delta_time;
        if self.fps_update <= 0.0 {
            self.fps_update += self.fps_update_time;
            self.current_fps = 1.0 / delta_time;
        }
    }

    pub fn draw(&mut self, view: UpdateView) {
        // clear_background(BLACK);
        self.draw_game(view);
        self.draw_ui();
    }

    fn draw_game(&mut self, view: UpdateView) {
        set_camera(&self.game_camera);

        // let offset = self.game_camera.world_to_screen(vec2(0.0, 0.0));
        // let offset = ivec2(offset.x as i32, offset.y as i32);
        let offset = ivec2(self.image.width as i32 / 2, 0);
        for (pos, tile) in view.tiles {
            let pos = pos + offset;
            if pos.x >= 0
                && pos.x < self.image.width as i32
                && pos.y >= 0
                && pos.y < self.image.height as i32
            {
                match tile {
                    Some(tile) => {
                        let color = tile_color(&tile);
                        self.image.set_pixel(pos.x as u32, pos.y as u32, color);
                    }
                    None => self.image.set_pixel(pos.x as u32, pos.y as u32, BLACK),
                }
            }
        }

        self.texture.update(&self.image);
        draw_texture(self.texture, -offset.x as f32, -offset.y as f32, WHITE);
    }

    fn draw_ui(&self) {
        set_default_camera(); // set_camera(&self.ui_camera);

        draw_text(
            &format!("FPS: {:.0}", self.current_fps),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
    }
}

fn tile_color(tile: &Tile) -> Color {
    match &tile.content {
        TileContent::Solid { tile_solid } => match tile_solid {
            TileSolid::Sand => GOLD,
            TileSolid::Barrier => WHITE,
        },
        TileContent::Liquid { tile_liquid } => match tile_liquid {
            TileLiquid::Water => BLUE,
        },
        TileContent::Gas { tile_gas } => match tile_gas {
            TileGas::Smoke => LIGHTGRAY,
        },
    }
}
