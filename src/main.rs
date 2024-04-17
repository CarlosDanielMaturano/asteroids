mod game_logic;
mod screen;
mod utils;
use crate::utils::space_object::SpaceObject;
use crate::utils::vector2::Vector2;
use game_logic::GameLogic;
use rand::random;
use rand::seq::SliceRandom;
use screen::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::f64::consts::PI;

const VERTS: usize = 20;
const DEFAULT_ASTEROID_SIZE: f64  = 24f64;

fn spawn_asteroid(asteroid_radius: f64, pos: Vector2) -> SpaceObject {
    let mut asteroid_model = [Vector2::empty(); VERTS];
    for (i, point) in asteroid_model.iter_mut().enumerate() {
        let radius = asteroid_radius + random::<f64>() * 5f64;
        let a = (i as f64 / VERTS as f64) * 2f64 * PI;
        *point = Vector2::new(radius * a.sin(), radius * a.cos());
    }

    let mut rng = rand::thread_rng();
    let dir = Vector2::new(
        random::<f64>() * 0.5f64,
        random::<f64>() * 0.5f64
    ) * Vector2::new(
        *[ -1f64, 1f64 ].choose(&mut rng).unwrap(),
        *[ -1f64, 1f64 ].choose(&mut rng).unwrap(),
    );
    let asteroid = SpaceObject {
        pos,
        dir,
        angle: random::<f64>() * 10f64,
        radius: asteroid_radius as usize,
        model: Box::new(asteroid_model),
    };
    asteroid
}

fn spawn_random_asteroids() -> Vec<SpaceObject> {
    let mut asteroids: Vec<SpaceObject> = Vec::new();
    for _ in 0..3 {
        let random_pos = Vector2::new(random::<f64>() * 512f64, random::<f64>() * 512_f64);
        asteroids.push(spawn_asteroid(DEFAULT_ASTEROID_SIZE, random_pos));
    }
    asteroids
}

fn spawn_bullet(origin: &SpaceObject) -> SpaceObject {
    let angle = origin.angle;
    SpaceObject {
        pos: origin.pos,
        dir: Vector2::new(5f64 * angle.sin(), -5f64 * angle.cos()),
        angle: origin.angle,
        model: Box::new([]),
        radius: 0
    }
}

fn is_point_inside_circle(cicle_pos: &Vector2, circle_radius: f64, point: &Vector2) -> bool {
    let Vector2 { x: cx, y: cy } = cicle_pos;
    let Vector2 { x, y } = point;
    ((x - cx).powf(2f64) + (y - cy).powf(2f64)).sqrt() < circle_radius
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
        radius: 0,
        model: Box::new(ship_model),
    };
    
    let mut player_bullets: Vec<SpaceObject> = Vec::new();

    let mut is_player_shooting = false;

    let mut asteroids = spawn_random_asteroids();


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
        if keys[Scancode::Space] {
            if !is_player_shooting {
                player_bullets.push(spawn_bullet(&player));
            }
            is_player_shooting = true;
        } else {
            is_player_shooting = false;
        }


        player_bullets.iter_mut().for_each(|bullet| {
            bullet.pos += bullet.dir * dt;
            screen.draw_pixel(bullet.pos, Color::WHITE)
        });

        player_bullets.retain(|bullet| {
            let ( x, y ) = bullet.pos.as_i32();
            if x < 1 || x > SCREEN_WIDTH as i32 || y < 1 || y > SCREEN_HEIGHT as i32{
                return false
            }
            true 
        });

        let mut dead_asteroid: Vec<(Vector2, f64)> = Vec::new();
        asteroids.retain(|asteroid| {
            for bullet in player_bullets.iter_mut() {
                let radius = asteroid.radius as f64;
                if is_point_inside_circle(&asteroid.pos, radius, &bullet.pos){
                    bullet.pos.x = -100.0;
                    dead_asteroid.push((asteroid.pos, radius));
                    return false
                }
            }
            !(asteroid.radius < 6)
        });
        dead_asteroid.iter().for_each(|asteroid| {
            let (pos, radius) = asteroid;
            for _ in 0..2 {
                asteroids.push(spawn_asteroid(radius / 2f64, *pos))
            }
        });

        if asteroids.is_empty() {
            asteroids = spawn_random_asteroids();
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
