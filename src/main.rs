mod screen;
mod utils;
use sdl2::event::Event;
use utils::Vector2;

fn main() {
    let mut screen = screen::Screen::new();
    let pos = Vector2(10.0, 10.0);
    let size = 16;
    while !screen.should_close {
        screen.clear();
        for event in screen.get_events().poll_iter() {
            match event {
                Event::Quit { .. } => screen.should_close = true,
                _ => ()
            }
        }

        for x in 0..size {
            for y in 0..size {
                screen.draw_pixel(Vector2(x as f64, y as f64) + pos, sdl2::pixels::Color::WHITE);
            }
        }

        screen.present();
    }
}
