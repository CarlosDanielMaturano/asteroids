mod screen;
mod utils;
mod game_logic;
use game_logic::GameLogic;
use utils::Vector2;
use sdl2::event::Event;
use sdl2::pixels::Color;

fn main() {
    let mut logic = GameLogic::new();

    logic.run(move |screen, _, dt| {
        screen.draw_line(Vector2(10.0, 10.0), Vector2(50.0, 40.0), Color::WHITE)
    });
}
