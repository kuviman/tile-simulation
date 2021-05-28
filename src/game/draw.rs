use super::*;

impl Game {
    pub fn draw(&self) {
        clear_background(BLACK);
        self.draw_game();
        self.draw_ui();
    }
    fn draw_game(&self) {
        set_camera(&self.game_camera);

        for (pos, tile) in &self.tiles {
            let color = tile_color(tile);
            draw_rectangle(pos.x as f32, pos.y as f32, 1.0, 1.0, color);
        }
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
