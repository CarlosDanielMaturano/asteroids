mod screen;
mod utils;
mod game_logic;
use game_logic::GameLogic;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use crate::utils::vector2::Vector2;
use crate::utils::space_object::SpaceObject;

fn main() {
    let mut logic = GameLogic::new();
    let mut player = SpaceObject {
        pos: Vector2::new(10.0, 10.0),
        dir: Vector2::new(0.0, 0.0),
        angle: 0f64,
        size: 1,
    };


    logic.run(move |screen, keys, dt| {
        if keys[Scancode::W] {
            player.dir.x += player.angle.sin() * 0.05 * dt;
            player.dir.y -= player.angle.cos() * 0.05 * dt;
        }

        if keys[Scancode::D] {
            player.angle += 0.05 * dt
        }
        if keys[Scancode::A] {
            player.angle -= 0.05 * dt
        }

        player.pos += player.dir * dt;
        player.pos.wrap();
    });
}
