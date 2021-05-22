use macroquad::color::colors;

use super::*;

impl Game {
    pub fn draw(&self) {
        clear_background(colors::BLACK);
        self.draw_game();
    }
    fn draw_game(&self) {
        set_camera(&self.game_camera);

        for (pos, tile) in &self.tiles {
            let color = tile_color(tile);
            draw_rectangle(pos.x as f32, pos.y as f32, 1.0, 1.0, color);
        }
    }
}

fn tile_color(tile: &Tile) -> Color {
    match &tile.content {
        TileContent::Solid { tile_solid } => match tile_solid {
            TileSolid::Sand => colors::GOLD,
            TileSolid::Barrier => colors::WHITE,
        },
        TileContent::Liquid { tile_liquid } => match tile_liquid {
            TileLiquid::Water => colors::BLUE,
        },
        TileContent::Gas { tile_gas } => match tile_gas {
            TileGas::Smoke => colors::LIGHTGRAY,
        },
    }
}
