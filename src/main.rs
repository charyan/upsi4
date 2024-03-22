mod drawing;
mod employee;

use std::time::Duration;

use employee::Office;
use macroquad::prelude::*;

struct Game {
    office: Office,
}

impl Game {
    pub fn new() -> Self {
        Self {
            office: Office::new(),
        }
    }
}

const FPS: f32 = 60.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let drawing = drawing::Drawing::new();
    let game = Game::new();

    loop {
        let start_time = get_time();

        drawing.draw(&game);
        next_frame().await;

        let elapsed_time = get_time() - start_time;
        let remaining_time = (1.0 / FPS) - elapsed_time as f32;

        if remaining_time > 0.0 {
            std::thread::sleep(Duration::from_secs_f64(remaining_time as f64));
        }
    }
}
