use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::utils::vector2::Vector2;

const WINDOW_TITLE: &str = "Asteroids";
const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;
const BACKGROUND_COLOR: Color = Color::BLACK;
const PIXEL_SIZE: u32 = 4;
pub const SCREEN_HEIGHT: u32= WINDOW_HEIGHT / PIXEL_SIZE;
pub const SCREEN_WIDTH: u32= WINDOW_WIDTH / PIXEL_SIZE;

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

    pub fn draw_pixel(&mut self, pos: Vector2, color: Color) {
        let scale = PIXEL_SIZE as i32;
        let mut pos = pos.clone();
        pos.wrap();
        let pos = pos.as_i32();
        let pixel = Rect::new(pos.0 * scale, pos.1 * scale, PIXEL_SIZE, PIXEL_SIZE);
        self.canvas.set_draw_color(color);
        if let Err(err) = self.canvas.fill_rect(pixel) {
            panic!("Unexpected error while drawing: {err}")
        };
    }

    pub fn draw_line(&mut self, start: Vector2, end: Vector2, color: Color) {
        let Vector2(delta_x, delta_y) = end - start;
        let (x_end, y_end) = end.as_i32();
        let mut err = 0.5;
        
        if delta_x.abs() > delta_y.abs() {
            let y_step = if delta_y < 0.0 { -1 } else { 1 };
            let mut slope = delta_y.abs() / delta_x.abs();
            if slope.is_nan() {
                slope = 0.0;
            }
            let (x_start, mut y) = start.as_i32();
            let range: Box<dyn Iterator<Item = i32>> = if delta_x > 0.0 {
                Box::new(x_start..=x_end)
            } else {
                Box::new((x_end..=x_start).rev())
            };
            for x in range {
                self.draw_pixel(Vector2(x as f64, y as f64), color);
                err += slope;
                if err >= 1.0 {
                    err -= 1.0;
                    y += y_step;
                }
            }
        } else {
            let x_step = if delta_x < 0.0 { -1 } else { 1 };
            let mut slope = delta_x.abs() / delta_y.abs();
            if slope.is_nan() {
                slope = 0.0
            }

            let (mut x, y_start) = start.as_i32();
            let range: Box<dyn Iterator<Item = i32>> = if delta_y > 0.0 {
                Box::new(y_start..=y_end)
            } else {
                Box::new((y_end..=y_start).rev())
            };

            for y in range {
                self.draw_pixel(Vector2(x as f64, y as f64), color);
                err += slope;
                if err >= 1.0 {
                    err -= 1.0;
                    x += x_step;
                }
            }
        }

    }
}
