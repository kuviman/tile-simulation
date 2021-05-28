use macroquad::prelude::*;
use std::time::Instant;

mod game;

use game::*;

const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;

#[macroquad::main("Tile Simulation")]
async fn main() {
    let mut game = Game::new();
    let mut frame_time = 0.0;
    loop {
        println!("---- next frame ----");
        let time = Instant::now();
        let delta_time = get_frame_time();
        frame_time += delta_time;
        game.update(delta_time);
        println!("update: {}ms", time.elapsed().as_millis());
        while frame_time >= FIXED_DELTA_TIME {
            game.fixed_update(FIXED_DELTA_TIME);
            frame_time -= FIXED_DELTA_TIME;
        }
        println!("fixed_update: {}ms", time.elapsed().as_millis());
        game.draw();
        println!("draw: {}ms", time.elapsed().as_millis());
        next_frame().await;
    }
}
