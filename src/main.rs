mod game_logic;
mod screen;
mod utils;
use crate::utils::space_object::SpaceObject;
use crate::utils::vector2::Vector2;
use game_logic::GameLogic;
use rand::random;
use rand::seq::SliceRandom;
use screen::{PIXEL_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::f32::consts::PI;

const VERTS: usize = 20;
const DEFAULT_ASTEROID_SIZE: f32 = 24f32;
const ASTEROID_SPAWN_COUNT: usize = 2;

fn spawn_asteroid(asteroid_radius: f32, pos: Vector2) -> SpaceObject {
    let mut asteroid_model = [Vector2::empty(); VERTS];
    for (i, point) in asteroid_model.iter_mut().enumerate() {
        let radius = asteroid_radius + random::<f32>() * 5f32;
        let a = (i as f32 / VERTS as f32) * 2f32 * PI;
        *point = Vector2::new(radius * a.sin(), radius * a.cos());
    }

    let mut rng = rand::thread_rng();
    let dir = Vector2::new(
        random::<f32>() * 0.3f32 * (DEFAULT_ASTEROID_SIZE / (asteroid_radius + 0.5f32)),
        random::<f32>() * 0.3f32 * (DEFAULT_ASTEROID_SIZE / (asteroid_radius + 0.5f32)),
    ) * Vector2::new(
        *[-1f32, 1f32].choose(&mut rng).unwrap(),
        *[-1f32, 1f32].choose(&mut rng).unwrap(),
    );
    let asteroid = SpaceObject {
        pos,
        dir,
        angle: random::<f32>() * 10f32,
        radius: asteroid_radius as usize,
        model: Box::new(asteroid_model),
    };
    asteroid
}

fn spawn_random_asteroids(player_angle: f32) -> Vec<SpaceObject> {
    let mut asteroids: Vec<SpaceObject> = Vec::new();
    for _ in 0..ASTEROID_SPAWN_COUNT {
        let random_pos = Vector2::new(
            random::<f32>() * 512_f32 * player_angle.cos(),
            random::<f32>() * 512_f32 * -player_angle.sin(),
        );
        asteroids.push(spawn_asteroid(DEFAULT_ASTEROID_SIZE, random_pos));
    }
    asteroids
}

fn spawn_bullet(origin: &SpaceObject) -> SpaceObject {
    let angle = origin.angle;
    SpaceObject {
        pos: origin.pos,
        dir: Vector2::new(5f32 * angle.sin(), -5f32 * angle.cos()),
        angle: origin.angle,
        model: Box::new([]),
        radius: 0,
    }
}

fn is_point_inside_circle(cicle_pos: &Vector2, circle_radius: f32, point: &Vector2) -> bool {
    let Vector2 { x: cx, y: cy } = cicle_pos;
    let Vector2 { x, y } = point;
    ((x - cx).powf(2f32) + (y - cy).powf(2f32)).sqrt() < circle_radius
}

fn main() {
    let mut logic = GameLogic::new();

    // Player
    let ship_model: [Vector2; 5] = [
        Vector2::new(0f32, -7f32),
        Vector2::new(-4f32, 3f32),
        Vector2::new(-2f32, 1f32),
        Vector2::new(2f32, 1f32),
        Vector2::new(4f32, 3f32),
    ];
    let mut player = SpaceObject {
        pos: Vector2::new(SCREEN_WIDTH as f32 / 2f32, SCREEN_HEIGHT as f32 / 2f32),
        dir: Vector2::empty(),
        angle: 0f32,
        radius: 0,
        model: Box::new(ship_model),
    };

    let mut player_bullets: Vec<SpaceObject> = Vec::new();

    let mut is_player_shooting = false;

    let mut asteroids = spawn_random_asteroids(player.angle);

    let mut player_score: usize = 0;

    logic.run(move |screen, keys, dt| {
        if keys[Scancode::W] || keys[Scancode::Up] {
            player.dir.x += player.angle.sin() * 0.06f32 * dt;
            player.dir.y -= player.angle.cos() * 0.06f32 * dt;
            let max_vel = 1.2f32;
            if player.dir.x.abs() >= max_vel {
                player.dir.x = (player.dir.x / player.dir.x.abs()) * max_vel
            }
            if player.dir.y.abs() >= max_vel {
                player.dir.y = (player.dir.y / player.dir.y.abs()) * max_vel
            }
        }

        if keys[Scancode::D] || keys[Scancode::Right] {
            player.angle += 0.1f32 * dt
        }
        if keys[Scancode::A] || keys[Scancode::Left] {
            player.angle -= 0.1f32 * dt
        }
        if keys[Scancode::Space] {
            if !is_player_shooting  && player_bullets.len() <= 5 {
                player_bullets.push(spawn_bullet(&player));
            }
            is_player_shooting = true;
        } else {
            is_player_shooting = false;
        }

        player_bullets.iter_mut().for_each(|bullet| {
            bullet.pos += bullet.dir * dt;
            screen.draw_pixel(bullet.pos, PIXEL_SIZE as i32, Color::WHITE)
        });

        player_bullets.retain_mut(|bullet| {
            let (x, y) = bullet.pos.as_i32();
            if x < 1 || x > SCREEN_WIDTH as i32 || y < 1 || y > SCREEN_HEIGHT as i32 {
                return false;
            }
            bullet.pos.wrap();
            true
        });

        let mut dead_asteroid: Vec<(Vector2, f32)> = Vec::new();
        asteroids.retain(|asteroid| {
            for bullet in player_bullets.iter_mut() {
                let radius = asteroid.radius as f32;
                if is_point_inside_circle(&asteroid.pos, radius, &bullet.pos) {
                    bullet.pos.x = -100f32;
                    player_score += 100 / asteroid.radius;
                    dead_asteroid.push((asteroid.pos, radius));
                    return false;
                }
            }
            !(asteroid.radius < 6)
        });
        dead_asteroid.iter().for_each(|asteroid| {
            let (pos, radius) = asteroid;
            for _ in 0..2 {
                asteroids.push(spawn_asteroid(radius / 2f32, *pos))
            }
        });

        if asteroids.is_empty() {
            asteroids = spawn_random_asteroids(player.angle);
            player_score += 1000;
        }

        asteroids.iter_mut().for_each(|asteroid| {
            asteroid.angle += 0.05f32 * dt;
            asteroid.pos += asteroid.dir * dt;
            asteroid.pos.wrap();
            screen.draw_wire_frame_model(&asteroid, Color::YELLOW);
        });

        player.pos += player.dir * dt;
        player.pos.wrap();

        screen.draw_wire_frame_model(&player, Color::WHITE);
        screen.draw_score(player_score, Color::WHITE);
    });
}
