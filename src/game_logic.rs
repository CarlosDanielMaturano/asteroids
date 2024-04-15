use crate::screen::Screen;
use sdl2::EventPump;
use std::time::{Duration, Instant};

const FPS_TARGET: u32 = 30;
const ONE_SECOND: u32 = Duration::from_secs(1).as_nanos() as u32;

pub struct GameLogic {
    screen: Screen,
}

impl GameLogic {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }
    pub fn run<T>(&mut self, mut logic: T)
    where
        T: FnMut(&mut Screen, &EventPump, f32),
    {
        let mut last_time = Instant::now();
        'gameloop: loop {
            let mut events = self.screen.get_events();
            for event in events.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'gameloop,
                    _ => (),
                }
            }
            self.screen.clear();

            let current_time = Instant::now();
            let delta_time = current_time.duration_since(last_time) * FPS_TARGET;
            last_time = current_time;

            logic(&mut self.screen, &events, delta_time.as_secs_f32());
            std::thread::sleep(Duration::new(0, ONE_SECOND / FPS_TARGET));

            self.screen.present();
        }
    }
}
