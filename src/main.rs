use macroquad::prelude::*;

const OFFICE_WIDTH: u32 = 16;
const OFFICE_HEIGHT: u32 = 9;

const GAME_WINDOW_WIDTH: u32 = 1280;
const GAME_WINDOW_HEIGHT: u32 = 720;

const GLOBAL_STAT_WIDTH: u32 = 50;
const GLOBAL_STAT_HEIGHT: u32 = 20;

const PERSONNAL_STAT_WIDTH: u32 = 50;
const PERSONNAL_STAT_HEIGHT: u32 = 20;

const INFO_WIDTH: u32 = 50;
const INFO_HEIGHT: u32 = 20;

fn draw_office(camera: &Camera2D) {
    set_camera(camera);
    clear_background(WHITE);
    draw_circle(7., 7., 1., RED);
}

fn draw_info(camera: &Camera2D) {
    set_camera(camera);
    clear_background(GREEN);
    draw_circle(7., 7., 1., RED);
}

fn draw_personnal_stat(camera: &Camera2D) {
    set_camera(camera);
    clear_background(BLUE);
    draw_circle(7., 7., 1., RED);
}

fn draw_global_stat(camera: &Camera2D) {
    set_camera(camera);
    clear_background(YELLOW);
    draw_circle(7., 7., 1., RED);
}

fn draw_game(
    camera: &Camera2D,
    texture_office: &Texture2D,
    texture_info: &Texture2D,
    texture_global_stat: &Texture2D,
    texture_personnal_stat: &Texture2D,
) {
    set_camera(camera);
    clear_background(LIGHTGRAY);

    draw_texture_ex(
        texture_global_stat,
        GAME_WINDOW_WIDTH as f32 * 0.3,
        GAME_WINDOW_HEIGHT as f32 * 0.01,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                GAME_WINDOW_WIDTH as f32 * 0.69,
                GAME_WINDOW_HEIGHT as f32 * 0.28,
            )),
            flip_y: true,
            ..Default::default()
        },
    );

    draw_texture_ex(
        texture_personnal_stat,
        GAME_WINDOW_WIDTH as f32 * 0.01,
        GAME_WINDOW_HEIGHT as f32 * 0.01,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                GAME_WINDOW_WIDTH as f32 * 0.28,
                GAME_WINDOW_HEIGHT as f32 * 0.28,
            )),
            flip_y: true,
            ..Default::default()
        },
    );

    draw_texture_ex(
        texture_info,
        GAME_WINDOW_WIDTH as f32 * 0.01,
        GAME_WINDOW_HEIGHT as f32 * 0.3,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                GAME_WINDOW_WIDTH as f32 * 0.28,
                GAME_WINDOW_HEIGHT as f32 * 0.69,
            )),
            flip_y: true,
            ..Default::default()
        },
    );

    draw_texture_ex(
        texture_office,
        GAME_WINDOW_WIDTH as f32 * 0.3,
        GAME_WINDOW_HEIGHT as f32 * 0.3,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                GAME_WINDOW_WIDTH as f32 * 0.69,
                GAME_WINDOW_HEIGHT as f32 * 0.69,
            )),
            flip_y: true,
            ..Default::default()
        },
    );
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Main target
    let main_render_target = render_target(GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT);
    main_render_target.texture.set_filter(FilterMode::Nearest);
    let mut main_camera = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        GAME_WINDOW_WIDTH as f32,
        GAME_WINDOW_HEIGHT as f32,
    ));
    main_camera.render_target = Some(main_render_target.clone());

    // Office target
    let render_target_office = render_target(OFFICE_WIDTH, OFFICE_HEIGHT);
    render_target_office.texture.set_filter(FilterMode::Nearest);
    let mut camera_office =
        Camera2D::from_display_rect(Rect::new(0., 0., OFFICE_WIDTH as f32, OFFICE_HEIGHT as f32));
    camera_office.render_target = Some(render_target_office.clone());

    // Info target
    let render_target_info = render_target(INFO_WIDTH, INFO_HEIGHT);
    render_target_info.texture.set_filter(FilterMode::Nearest);
    let mut camera_info =
        Camera2D::from_display_rect(Rect::new(0., 0., INFO_WIDTH as f32, INFO_HEIGHT as f32));
    camera_info.render_target = Some(render_target_info.clone());

    // Global stat target
    let render_target_global_stat = render_target(GLOBAL_STAT_WIDTH, GLOBAL_STAT_HEIGHT);
    render_target_global_stat
        .texture
        .set_filter(FilterMode::Nearest);
    let mut camera_global_stat = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        GLOBAL_STAT_WIDTH as f32,
        GLOBAL_STAT_HEIGHT as f32,
    ));
    camera_global_stat.render_target = Some(render_target_global_stat.clone());

    // Personnal stat target
    let render_target_personnal_stat = render_target(PERSONNAL_STAT_WIDTH, PERSONNAL_STAT_HEIGHT);
    render_target_personnal_stat
        .texture
        .set_filter(FilterMode::Nearest);
    let mut camera_personnal_stat = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        PERSONNAL_STAT_WIDTH as f32,
        PERSONNAL_STAT_HEIGHT as f32,
    ));
    camera_personnal_stat.render_target = Some(render_target_personnal_stat.clone());

    loop {
        clear_background(BLACK);
        draw_office(&camera_office);
        draw_info(&camera_info);
        draw_global_stat(&camera_global_stat);
        draw_personnal_stat(&camera_personnal_stat);

        draw_game(
            &main_camera,
            &render_target_office.texture,
            &render_target_info.texture,
            &render_target_global_stat.texture,
            &render_target_personnal_stat.texture,
        );

        set_default_camera();

        let height = screen_width() / 16. * 9.;

        draw_texture_ex(
            &main_render_target.texture,
            0.,
            (screen_height() - height) / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_width() / 16. * 9.)),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
