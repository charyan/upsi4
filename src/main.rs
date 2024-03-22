mod drawing;
mod employee;

use std::time::Duration;

use drawing::{GAME_WINDOW_HEIGHT, GAME_WINDOW_WIDTH};
use employee::Office;
use macroquad::prelude::*;

struct Game {
    office: Office,
    displayed_hope: f32,
    displayed_satiety: f32,
    displayed_satisfaction: f32,
    displayed_energy: f32,
}

impl Game {
    pub fn new() -> Self {
        let mut new_office = Office::new();
        new_office.add_employee();
        Self {
            office: new_office,
            displayed_energy: 0.,
            displayed_hope: 0.,
            displayed_satiety: 0.,
            displayed_satisfaction: 0.,
        }
    }

    pub fn tick(&mut self) {
        self.office.tick();
    }

    pub fn get_displayed_energy(&self) -> f32 {
        self.displayed_energy
    }

    pub fn get_displayed_satiety(&self) -> f32 {
        self.displayed_satiety
    }

    pub fn get_displayed_satisfaction(&self) -> f32 {
        self.displayed_satisfaction
    }

    pub fn get_displayed_hope(&self) -> f32 {
        self.displayed_hope
    }

    pub fn get_office(&self) -> &Office {
        &self.office
    }

    pub fn get_mut_office(&mut self) -> &mut Office {
        &mut self.office
    }

    pub fn clicked(&mut self) {
        self.displayed_energy = 0.;
        self.displayed_hope = 0.;
        self.displayed_satiety = 0.;
        self.displayed_satisfaction = 0.;
    }
}

const FPS: f32 = 60.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let drawing = drawing::Drawing::new();
    let mut game = Game::new();

    loop {
        let start_time = get_time();

        if is_mouse_button_pressed(MouseButton::Left) {
            let main_pos =
                drawing.convert_screen_main(vec2(mouse_position().0, mouse_position().1));
            let rect_office = Rect::new(
                GAME_WINDOW_WIDTH as f32 * 0.3,
                GAME_WINDOW_HEIGHT as f32 * 0.3,
                GAME_WINDOW_WIDTH as f32 * 0.69,
                GAME_WINDOW_HEIGHT as f32 * 0.69,
            );

            if rect_office.contains(main_pos) {
                game.get_mut_office().click(
                    drawing.convert_screen_office(vec2(mouse_position().0, mouse_position().1)),
                );
                game.clicked();
            }
        }

        if is_key_pressed(KeyCode::Space) {
            game.get_mut_office().add_employee();
        }

        game.tick();
        drawing.draw(&game);

        next_frame().await;

        let elapsed_time = get_time() - start_time;
        let remaining_time = (1.0 / FPS) - elapsed_time as f32;

        if remaining_time > 0.0 {
            std::thread::sleep(Duration::from_secs_f64(remaining_time as f64));
        }
    }
}
