mod game_logic;
mod screen;
mod utils;
use crate::utils::space_object::SpaceObject;
use crate::utils::vector2::Vector2;
use game_logic::GameLogic;
use rand::random;
use rand::seq::SliceRandom;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::f64::consts::PI;

const VERTS: usize = 20;
const BIG: f64  = 20f64;
const MEDIUM:  f64 = 12f64;

fn spawn_asteroid() -> SpaceObject {
    let mut asteroid_model = [Vector2::empty(); VERTS];
    let asteroid_radius = *[BIG, MEDIUM]
        .choose(&mut rand::thread_rng())
        .unwrap();
    for (i, point) in asteroid_model.iter_mut().enumerate() {
        let radius = asteroid_radius + random::<f64>() * 5f64;
        let a = (i as f64 / VERTS as f64) * 2f64 * PI;
        *point = Vector2::new(radius * a.sin(), radius * a.cos());
    }

    let asteroid = SpaceObject {
        pos: Vector2::new(random::<f64>() * 512f64, random::<f64>() * 512_f64),
        dir: Vector2::new(random::<f64>() * 0.5f64, random::<f64>() * 0.5f64),
        angle: random::<f64>() * 10f64,
        size: asteroid_radius as usize,
        model: Box::new(asteroid_model),
    };
    asteroid
}

fn main() {
    let mut logic = GameLogic::new();

    // Player
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


    // Asteroids
    let mut asteroids: Vec<SpaceObject> = Vec::new();
    for _ in 0..3 {
        asteroids.push(spawn_asteroid());
    }

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

        asteroids.iter_mut().for_each(|asteroid| {
            asteroid.angle += 0.05f64 * dt;
            asteroid.pos += asteroid.dir * dt;
            asteroid.pos.wrap();
            screen.draw_wire_frame_model(&asteroid, Color::YELLOW);
        });

        player.pos += player.dir * dt;
        player.pos.wrap();

        screen.draw_wire_frame_model(&player, Color::GREEN);
    });
}
