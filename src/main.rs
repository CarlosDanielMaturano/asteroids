mod screen;
mod utils;
mod game_logic;
use game_logic::GameLogic;
use sdl2::pixels::Color;
use crate::utils::vector2::Vector2;
use crate::utils::space_object::SpaceObject;

fn main() {
    let mut logic = GameLogic::new();
    let mut player = SpaceObject {
        pos: Vector2::new(10.0, 10.0),
        dir: Vector2::new(0.5f64, -0.5f64),
        angle: 0f64,
        size: 16,
    };

    logic.run(move |screen, _, dt| {
        for x in 0..player.size {
            for y in 0..player.size {
                let pos = Vector2::new(x as f64, y as f64) + player.pos;
                screen.draw_pixel(pos, Color::WHITE)
            }
        }
        player.pos += player.dir * dt;
        player.pos.wrap();
    });
}
