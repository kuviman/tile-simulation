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
        let delta_time = get_frame_time();
        frame_time += delta_time;
        let time = Instant::now();
        game.update(delta_time);
        println!("update: {}ms", time.elapsed().as_millis());
        let time = Instant::now();
        let mut frames = 0;
        while frame_time >= FIXED_DELTA_TIME {
            game.fixed_update(FIXED_DELTA_TIME);
            frame_time -= FIXED_DELTA_TIME;
            frames += 1;
        }
        println!(
            "fixed_update: {}ms / {} frames",
            time.elapsed().as_millis(),
            frames
        );
        let time = Instant::now();
        game.draw();
        println!("draw: {}ms", time.elapsed().as_millis());
        next_frame().await;
    }
}
