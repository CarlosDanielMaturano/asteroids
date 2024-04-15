mod screen;
mod utils;
use sdl2::event::Event;
use sdl2::pixels::Color;
use utils::Vector2;

fn main() {
    let mut screen = screen::Screen::new();
    let end = Vector2(32.0, 32.0);
    while !screen.should_close {
        screen.clear();
        let mut events = screen.get_events();
        let mouse = sdl2::mouse::MouseState::new(&events);
        let mouse_pos = Vector2((mouse.x() as f64/ 8.0).round(), (mouse.y() as f64 / 8.0).round());
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => screen.should_close = true,
                _ => (),
            }
        }
    
        screen.draw_line(end, mouse_pos, Color::WHITE);

        screen.present();
    }
}
