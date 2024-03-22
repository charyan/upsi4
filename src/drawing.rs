use macroquad::prelude::*;

use crate::{employee::EMPLOYEE_RADIUS, Game};

pub const OFFICE_WIDTH: u32 = 160;
pub const OFFICE_HEIGHT: u32 = 90;

pub const GAME_WINDOW_WIDTH: u32 = 1280;
pub const GAME_WINDOW_HEIGHT: u32 = 720;

pub const GLOBAL_STAT_WIDTH: u32 = 50;
pub const GLOBAL_STAT_HEIGHT: u32 = 20;

pub const PERSONNAL_STAT_WIDTH: u32 = 16;
pub const PERSONNAL_STAT_HEIGHT: u32 = 9;

pub const INFO_WIDTH: u32 = 50;
pub const INFO_HEIGHT: u32 = 20;

pub const ANIMATION_SPEED: f32 = 0.1;

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start.mul_add(1.0 - t, end * t)
}

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

    fn draw_office(&self, game: &Game) {
        set_camera(&self.camera_office);
        clear_background(WHITE);
        for e in game.get_office().iter_employees() {
            draw_circle(e.get_pos().x, e.get_pos().y, EMPLOYEE_RADIUS, RED);
        }
    }

    fn draw_info(&self) {
        set_camera(&self.camera_info);
        clear_background(WHITE);
    }

    fn draw_personnal_stat(&self, game: &Game) {
        set_camera(&self.camera_personnal_stat);
        clear_background(WHITE);

        let bar_width_max: f32 = 10.;

        if let Some(selected_employee) = game.get_office().get_selected_employee() {
            draw_rectangle(1., 1., bar_width_max, 1., LIGHTGRAY);
            draw_rectangle(1., 3., bar_width_max, 1., LIGHTGRAY);
            draw_rectangle(1., 5., bar_width_max, 1., LIGHTGRAY);
            draw_rectangle(1., 7., bar_width_max, 1., LIGHTGRAY);

            draw_rectangle(
                1.,
                1.,
                lerp(
                    game.get_displayed_satisfaction(),
                    selected_employee.as_ref().borrow().get_satisfaction() * bar_width_max,
                    ANIMATION_SPEED,
                ),
                1.,
                RED,
            );
            draw_rectangle(
                1.,
                3.,
                lerp(
                    game.get_displayed_energy(),
                    selected_employee.as_ref().borrow().get_energy() * bar_width_max,
                    ANIMATION_SPEED,
                ),
                1.,
                YELLOW,
            );
            draw_rectangle(
                1.,
                5.,
                lerp(
                    game.get_displayed_satiety(),
                    selected_employee.as_ref().borrow().get_satiety() * bar_width_max,
                    ANIMATION_SPEED,
                ),
                1.,
                BLUE,
            );
            draw_rectangle(
                1.,
                7.,
                lerp(
                    game.get_displayed_hope(),
                    selected_employee.as_ref().borrow().get_hope() * bar_width_max,
                    ANIMATION_SPEED,
                ),
                1.,
                GREEN,
            );
        }
    }

    fn draw_global_stat(&self) {
        set_camera(&self.camera_global_stat);
        clear_background(WHITE);
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
        self.draw_office(game);
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
                dest_size: Some(vec2(screen_width(), height)),
                flip_y: true,
                ..Default::default()
            },
        );
    }

    pub fn convert_screen_main(&self, coords: Vec2) -> Vec2 {
        let height = screen_width() / 16. * 9.;
        let y = (screen_height() - height) / 2.;
        vec2(
            ((coords.x - 0.) / screen_width()) * GAME_WINDOW_WIDTH as f32,
            ((coords.y - y) / height) * GAME_WINDOW_HEIGHT as f32,
        )
    }

    pub fn convert_screen_office(&self, coords: Vec2) -> Vec2 {
        let main_coords = self.convert_screen_main(coords);

        let x = GAME_WINDOW_WIDTH as f32 * 0.3;
        let width = GAME_WINDOW_WIDTH as f32 * 0.69;

        let height = GAME_WINDOW_HEIGHT as f32 * 0.69;
        let y = GAME_WINDOW_HEIGHT as f32 * 0.3;

        vec2(
            ((main_coords.x - x) / width) * OFFICE_WIDTH as f32,
            ((main_coords.y - y) / height) * OFFICE_HEIGHT as f32,
        )
    }
}
