mod drawing;

use macroquad::prelude::*;

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

    loop {
        drawing.draw();
        next_frame().await
    }
}
