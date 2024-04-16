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
        size: 0,
    };

    let ship_model: [Vector2; 3] = [
        Vector2::new(0.0,  -6.5),
        Vector2::new(-2.5, 3.5),
        Vector2::new(2.5,  3.5),
    ];


    logic.run(move |screen, keys, dt| {

        let mut model = ship_model; 

        for (i, point) in ship_model.iter().enumerate() {
            // rotate
            model[i] = Vector2::new(
                point.x * player.angle.cos() - point.y * player.angle.sin(),
                point.x * player.angle.sin() + point.y * player.angle.cos(),
            );
            // translate
            model[i] += player.pos
        }

        let k = ship_model.len();
        for i in 0..=k {
            let j = i + 1;
            screen.draw_line(
                model[i % k],
                model[j % k],
                Color::WHITE
            )
        }

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
