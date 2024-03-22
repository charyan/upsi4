use macroquad::prelude::*;

use crate::Game;

const OFFICE_WIDTH: u32 = 16;
const OFFICE_HEIGHT: u32 = 9;

const GAME_WINDOW_WIDTH: u32 = 1280;
const GAME_WINDOW_HEIGHT: u32 = 720;

const GLOBAL_STAT_WIDTH: u32 = 50;
const GLOBAL_STAT_HEIGHT: u32 = 20;

const PERSONNAL_STAT_WIDTH: u32 = 16;
const PERSONNAL_STAT_HEIGHT: u32 = 9;

const INFO_WIDTH: u32 = 50;
const INFO_HEIGHT: u32 = 20;

pub struct Drawing {
    // Render_target
    main_render_target: RenderTarget,
    render_target_office: RenderTarget,
    render_target_info: RenderTarget,
    render_target_global_stat: RenderTarget,
    render_target_personnal_stat: RenderTarget,

    // Camera
    main_camera: Camera2D,
    camera_office: Camera2D,
    camera_info: Camera2D,
    camera_global_stat: Camera2D,
    camera_personnal_stat: Camera2D,
}

impl Drawing {
    pub fn new() -> Self {
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
        let mut camera_office = Camera2D::from_display_rect(Rect::new(
            0.,
            0.,
            OFFICE_WIDTH as f32,
            OFFICE_HEIGHT as f32,
        ));
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
        let render_target_personnal_stat =
            render_target(PERSONNAL_STAT_WIDTH, PERSONNAL_STAT_HEIGHT);
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

        Self {
            main_render_target,
            render_target_office,
            render_target_info,
            render_target_global_stat,
            render_target_personnal_stat,

            // Camera
            main_camera,
            camera_office,
            camera_info,
            camera_global_stat,
            camera_personnal_stat,
        }
    }

    fn draw_office(&self) {
        set_camera(&self.camera_office);
        clear_background(WHITE);
        draw_circle(7., 7., 1., RED);
    }

    fn draw_info(&self) {
        set_camera(&self.camera_info);
        clear_background(GREEN);
        draw_circle(7., 7., 1., RED);
    }

    fn draw_personnal_stat(&self, game: &Game) {
        set_camera(&self.camera_personnal_stat);
        clear_background(WHITE);

        let bar_width_max: f32 = 10.;

        let selected_employee = 0.3;

        draw_rectangle(1., 1., bar_width_max, 1., LIGHTGRAY);
        draw_rectangle(1., 3., bar_width_max, 1., LIGHTGRAY);
        draw_rectangle(1., 5., bar_width_max, 1., LIGHTGRAY);
        draw_rectangle(1., 7., bar_width_max, 1., LIGHTGRAY);

        draw_rectangle(1., 1., bar_width_max * selected_employee, 1., RED);
        draw_rectangle(1., 3., bar_width_max * selected_employee, 1., RED);
        draw_rectangle(1., 5., bar_width_max * selected_employee, 1., RED);
        draw_rectangle(1., 7., bar_width_max * selected_employee, 1., RED);
    }

    fn draw_global_stat(&self) {
        set_camera(&self.camera_global_stat);
        clear_background(YELLOW);
        draw_circle(7., 7., 1., RED);
    }

    fn draw_game(&self) {
        set_camera(&self.main_camera);
        clear_background(LIGHTGRAY);

        draw_texture_ex(
            &self.render_target_global_stat.texture,
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
            &self.render_target_personnal_stat.texture,
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
            &self.render_target_info.texture,
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
            &self.render_target_office.texture,
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

    pub fn draw(&self, game: &Game) {
        self.draw_global_stat();
        self.draw_personnal_stat(game);
        self.draw_office();
        self.draw_info();
        self.draw_game();

        set_default_camera();
        clear_background(BLACK);

        let height = screen_width() / 16. * 9.;

        draw_texture_ex(
            &self.main_render_target.texture,
            0.,
            (screen_height() - height) / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_width() / 16. * 9.)),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}
