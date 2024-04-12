use sdl2::pixels::Color;

const WINDOW_TITLE: &str = "Asteroids";
const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;
const BACKGROUND_COLOR: Color = Color::BLACK;


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
            should_close: false
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
}
