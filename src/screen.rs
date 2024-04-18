use crate::utils::font;
use crate::utils::space_object::SpaceObject;
use crate::utils::vector2::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WINDOW_TITLE: &str = "Asteroids";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 640;
const BACKGROUND_COLOR: Color = Color::BLACK;
pub const PIXEL_SIZE: i32 = 4;
pub const SCREEN_HEIGHT: u32 = WINDOW_HEIGHT / PIXEL_SIZE as u32;
pub const SCREEN_WIDTH: u32 = WINDOW_WIDTH / PIXEL_SIZE as u32;

pub struct Screen {
    ctx: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub should_close: bool,
}

impl Screen {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("sdl2 context");
        let video_subsystem = sdl_context.video().expect("sdl2 video subsystem");
        let window = video_subsystem
            .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("sdl2 window");
        let canvas = window.into_canvas().build().expect("sdl2 canvas");
        Self {
            ctx: sdl_context,
            canvas,
            should_close: false,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn get_events(&mut self) -> sdl2::EventPump {
        self.ctx.event_pump().unwrap()
    }

    pub fn draw_pixel(&mut self, pos: Vector2, scale: i32, color: Color) {
        let mut pos = pos.clone();
        pos.wrap();
        let pos = pos.as_i32();
        let pixel = Rect::new(
            pos.0 * scale,
            pos.1 * scale,
            PIXEL_SIZE as u32,
            PIXEL_SIZE as u32,
        );
        self.canvas.set_draw_color(color);
        if let Err(err) = self.canvas.fill_rect(pixel) {
            panic!("Unexpected error while drawing: {err}")
        };
    }

    pub fn draw_line(&mut self, start: Vector2, end: Vector2, color: Color) {
        let Vector2 {
            x: delta_x,
            y: delta_y,
        } = end - start;
        let (x_end, y_end) = end.as_i32();
        let mut err = 0.5f32;

        if delta_x.abs() > delta_y.abs() {
            let y_step = if delta_y < 0f32 { -1 } else { 1 };
            let mut slope = delta_y.abs() / delta_x.abs();
            if slope.is_nan() {
                slope = 0f32;
            }
            let (x_start, mut y) = start.as_i32();
            let range: Box<dyn Iterator<Item = i32>> = if delta_x > 0f32 {
                Box::new(x_start..=x_end)
            } else {
                Box::new((x_end..=x_start).rev())
            };
            for x in range {
                self.draw_pixel(Vector2::new(x as f32, y as f32), PIXEL_SIZE, color);

                err += slope;
                if err >= 1f32 {
                    err -= 1f32;
                    y += y_step;
                }
            }
        } else {
            let x_step = if delta_x < 0f32 { -1 } else { 1 };
            let mut slope = delta_x.abs() / delta_y.abs();
            if slope.is_nan() {
                slope = 0f32
            }

            let (mut x, y_start) = start.as_i32();
            let range: Box<dyn Iterator<Item = i32>> = if delta_y > 0f32 {
                Box::new(y_start..=y_end)
            } else {
                Box::new((y_end..=y_start).rev())
            };

            for y in range {
                self.draw_pixel(Vector2::new(x as f32, y as f32), PIXEL_SIZE, color);
                err += slope;
                if err >= 1f32 {
                    err -= 1f32;
                    x += x_step;
                }
            }
        }
    }

    pub fn draw_wire_frame_model(&mut self, object: &SpaceObject, color: Color) {
        let mut model = object.model.to_owned();
        let cos = object.angle.cos();
        let sin = object.angle.sin();
        for point in model.iter_mut() {
            // rotate
            *point = Vector2::new(point.x * cos - point.y * sin, point.x * sin + point.y * cos);
            // translate
            *point += object.pos;
        }

        let size = model.len();
        for i in 0..=size {
            let j = i + 1;
            self.draw_line(model[i % size], model[j % size], color)
        }
    }

    pub fn draw_score(&mut self, score: usize, color: Color) {
        let digits: Vec<usize> = score
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect();
        let x_space = font::FONT_WIDHT + 3;
        let y_space = font::FONT_HEIGHT + 3;
        let initial_pos = Vector2::new(10f32, 0f32);
        let space = Vector2::new(x_space as f32, y_space as f32);
        for (i, digit) in digits.iter().enumerate() {
            let digit_font_char = font::DIGITS_FONTS[*digit];
            for (y, row) in digit_font_char.iter().enumerate() {
                for (x, pixel) in row.iter().enumerate() {
                    if *pixel == 0 {
                        continue;
                    }
                    let mut offset = space;
                    offset.x *= i as f32;
                    self.draw_pixel(
                        initial_pos + Vector2::new(x as f32, y as f32) + offset,
                        2i32,
                        color,
                    )
                }
            }
        }
    }
}
