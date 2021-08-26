use macroquad::prelude::*;

mod constants;
mod game;
mod update_view;

use game::Game;

const FIXED_DELTA_TIME: f32 = 1.0 / 30.0;
const MIN_UPDATES_PER_FRAME: usize = 1;
const MAX_UPDATES_PER_FRAME: usize = 1;

#[macroquad::main("Tile Physics")]
async fn main() {
    assert!(MIN_UPDATES_PER_FRAME <= MAX_UPDATES_PER_FRAME);

    let mut game = Game::new();
    let mut frame_time = 0.0;
    let mut paused = true;
    loop {
        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }

        if !paused || is_key_pressed(KeyCode::Space) {
            let delta_time = get_frame_time();
            frame_time += delta_time;
            game.update(delta_time);
            let mut updates = 0;
            while updates < MIN_UPDATES_PER_FRAME
                || frame_time >= FIXED_DELTA_TIME && updates < MAX_UPDATES_PER_FRAME
            {
                game.fixed_update(FIXED_DELTA_TIME);
                frame_time -= FIXED_DELTA_TIME;
                updates += 1;
            }
        }

        game.draw();
        next_frame().await;
    }
}
