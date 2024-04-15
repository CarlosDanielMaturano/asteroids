mod screen;
mod utils;
mod game_logic;
use game_logic::GameLogic;
use utils::Vector2;
use sdl2::pixels::Color;

fn main() {
    let mut logic = GameLogic::new();
    let mut player_pos = Vector2(10.0, 10.0);
    let player_dir = Vector2(1f64, 0.7f64);
    let player_size = 16;

    logic.run(move |screen, _, dt| {
        for x in 0..player_size {
            for y in 0..player_size {
                let pos = Vector2(x as f64, y as f64) + player_pos;
                screen.draw_pixel(pos, Color::WHITE)
            }
        }
        player_pos += player_dir * dt;
        player_pos.wrap();
    });
}
