mod game_logic;
mod screen;
mod utils;
use crate::utils::space_object::SpaceObject;
use crate::utils::vector2::Vector2;
use game_logic::GameLogic;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;

fn main() {
    let mut logic = GameLogic::new();

    let ship_model: [Vector2; 5] = [
        Vector2::new(0.0, -7.0), 
        Vector2::new(-4.0, 3.0), 
        Vector2::new(-2.0, 1.0), 
        Vector2::new(2.0, 1.0),  
        Vector2::new(4.0, 3.0),  
    ];

    let mut player = SpaceObject {
        pos: Vector2::new(32.0, 32.0),
        dir: Vector2::new(0.0, 0.0),
        angle: 0f64,
        size: 0,
        model: Box::new(ship_model),
    };

    logic.run(move |screen, keys, dt| {
        if keys[Scancode::W] {
            player.dir.x += player.angle.sin() * 0.05 * dt;
            player.dir.y -= player.angle.cos() * 0.05 * dt;
        }

        if keys[Scancode::D] {
            player.angle += 0.07 * dt
        }
        if keys[Scancode::A] {
            player.angle -= 0.07 * dt
        }

        player.pos += player.dir * dt;
        player.pos.wrap();

        screen.draw_wire_frame_model(&player, &ship_model, Color::GREEN);
    });
}
