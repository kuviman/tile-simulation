use macroquad::prelude::{get_frame_time, next_frame};

mod game;

use game::*;

const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;

#[macroquad::main("Tile Simulation")]
async fn main() {
    let mut game = Game::new();
    let mut frame_time = 0.0;
    loop {
        let delta_time = get_frame_time();
        frame_time += delta_time;
        game.update(delta_time);
        while frame_time >= FIXED_DELTA_TIME {
            game.fixed_update(FIXED_DELTA_TIME);
            frame_time -= FIXED_DELTA_TIME;
        }
        game.draw();
        next_frame().await;
    }
}