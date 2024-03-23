#[warn(clippy::pedantic, clippy::nursery)]
#[allow(clippy::cast_precision_loss)]
mod drawing;
mod employee;

use drawing::{Drawing, GAME_WINDOW_HEIGHT, GAME_WINDOW_WIDTH};
use employee::Office;
use macroquad::{experimental::coroutines::wait_seconds, prelude::*};

struct Game {
    office: Office,
}

impl Game {
    pub fn new() -> Self {
        let mut new_office = Office::new();
        new_office.add_employee();
        Self { office: new_office }
    }

    pub fn tick(&mut self) {
        self.office.tick();
    }

    pub fn get_office(&self) -> &Office {
        &self.office
    }

    pub fn get_mut_office(&mut self) -> &mut Office {
        &mut self.office
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
    let mut drawing = drawing::Drawing::new();
    let mut game = Game::new();

    let rect_office = Rect::new(
        GAME_WINDOW_WIDTH as f32 * 0.3,
        GAME_WINDOW_HEIGHT as f32 * 0.3,
        GAME_WINDOW_WIDTH as f32 * 0.69,
        GAME_WINDOW_HEIGHT as f32 * 0.69,
    );

    let rect_info = Rect::new(
        GAME_WINDOW_WIDTH as f32 * 0.01,
        GAME_WINDOW_HEIGHT as f32 * 0.3,
        GAME_WINDOW_WIDTH as f32 * 0.28,
        GAME_WINDOW_HEIGHT as f32 * 0.69,
    );

    let rect_personnal_stat = Rect::new(
        GAME_WINDOW_WIDTH as f32 * 0.01,
        GAME_WINDOW_HEIGHT as f32 * 0.01,
        GAME_WINDOW_WIDTH as f32 * 0.28,
        GAME_WINDOW_HEIGHT as f32 * 0.28,
    );

    let rect_global_stat = Rect::new(
        GAME_WINDOW_WIDTH as f32 * 0.3,
        GAME_WINDOW_HEIGHT as f32 * 0.01,
        GAME_WINDOW_WIDTH as f32 * 0.69,
        GAME_WINDOW_HEIGHT as f32 * 0.28,
    );

    loop {
        let start_time = get_time();

        if is_mouse_button_pressed(MouseButton::Left) {
            let main_pos =
                Drawing::convert_screen_main(vec2(mouse_position().0, mouse_position().1));
            let rect_office = Rect::new(
                GAME_WINDOW_WIDTH as f32 * 0.3,
                GAME_WINDOW_HEIGHT as f32 * 0.3,
                GAME_WINDOW_WIDTH as f32 * 0.69,
                GAME_WINDOW_HEIGHT as f32 * 0.69,
            );

            if rect_office.contains(main_pos) {
                game.get_mut_office()
                    .click(Drawing::convert_main_office(main_pos));
                drawing.reset_displayed();
            } else if rect_info.contains(main_pos) {
                println!("{:?}", drawing.convert_main_info(main_pos))
            } else if rect_global_stat.contains(main_pos) {
                println!("{:?}", drawing.convert_main_global_stat(main_pos))
            } else if rect_personnal_stat.contains(main_pos) {
                println!("{:?}", drawing.convert_main_personnal_stat(main_pos))
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
            wait_seconds(remaining_time).await;
        }
    }
}
