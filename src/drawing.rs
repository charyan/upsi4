use macroquad::prelude::*;

use crate::{employee::EMPLOYEE_RADIUS, Game};

pub const OFFICE_WIDTH: u32 = 1280;
pub const OFFICE_HEIGHT: u32 = 720;

pub const GAME_WINDOW_WIDTH: u32 = 1280;
pub const GAME_WINDOW_HEIGHT: u32 = 720;

pub const GLOBAL_STAT_WIDTH: u32 = 1600;
pub const GLOBAL_STAT_HEIGHT: u32 = 900;

pub const PERSONNAL_STAT_WIDTH: u32 = 1600;
pub const PERSONNAL_STAT_HEIGHT: u32 = 900;

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
    // Displayed
    displayed_hope: f32,
    displayed_satiety: f32,
    displayed_satisfaction: f32,
    displayed_energy: f32,

    // Button
    button_personnal_hope: Rect,
    button_personnal_satiety: Rect,
    button_personnal_energy: Rect,
    button_personnal_satisfaction: Rect,

    // Rect render
    rect_office: Rect,
    rect_info: Rect,
    rect_personnal_stat: Rect,
    rect_global_stat: Rect,

    // Texture
    employee_texture: Texture2D,
    office_texture: Texture2D,
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

        let employee_texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/employees/employee0_normal.png"),
            Some(ImageFormat::Png),
        );

        let office_texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/office/office.png"),
            Some(ImageFormat::Png),
        );

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

            // displayed
            displayed_energy: 0.,
            displayed_hope: 0.,
            displayed_satiety: 0.,
            displayed_satisfaction: 0.,

            // Button
            button_personnal_satisfaction: Rect::new(1200., 75., 300., 150.),
            button_personnal_energy: Rect::new(1200., 275., 300., 150.),
            button_personnal_satiety: Rect::new(1200., 475., 300., 150.),
            button_personnal_hope: Rect::new(1200., 675., 300., 150.),

            //Render rect
            rect_office: Rect::new(
                GAME_WINDOW_WIDTH as f32 * 0.3,
                GAME_WINDOW_HEIGHT as f32 * 0.3,
                GAME_WINDOW_WIDTH as f32 * 0.69,
                GAME_WINDOW_HEIGHT as f32 * 0.69,
            ),
            rect_info: Rect::new(
                GAME_WINDOW_WIDTH as f32 * 0.01,
                GAME_WINDOW_HEIGHT as f32 * 0.3,
                GAME_WINDOW_WIDTH as f32 * 0.28,
                GAME_WINDOW_HEIGHT as f32 * 0.69,
            ),
            rect_personnal_stat: Rect::new(
                GAME_WINDOW_WIDTH as f32 * 0.01,
                GAME_WINDOW_HEIGHT as f32 * 0.01,
                GAME_WINDOW_WIDTH as f32 * 0.28,
                GAME_WINDOW_HEIGHT as f32 * 0.28,
            ),
            rect_global_stat: Rect::new(
                GAME_WINDOW_WIDTH as f32 * 0.3,
                GAME_WINDOW_HEIGHT as f32 * 0.01,
                GAME_WINDOW_WIDTH as f32 * 0.69,
                GAME_WINDOW_HEIGHT as f32 * 0.28,
            ),

            employee_texture,
            office_texture,
        }
    }

    pub fn get_rect_office(&self) -> &Rect {
        &self.rect_office
    }

    pub fn get_rect_global_stat(&self) -> &Rect {
        &self.rect_global_stat
    }

    pub fn get_rect_personnal_stat(&self) -> &Rect {
        &self.rect_personnal_stat
    }

    pub fn get_rect_info(&self) -> &Rect {
        &self.rect_info
    }

    pub fn get_button_hope(&self) -> &Rect {
        &self.button_personnal_hope
    }

    pub fn get_button_satiety(&self) -> &Rect {
        &self.button_personnal_satiety
    }

    pub fn get_button_energy(&self) -> &Rect {
        &self.button_personnal_energy
    }

    pub fn get_button_satisfaction(&self) -> &Rect {
        &self.button_personnal_satisfaction
    }

    fn draw_office(&self, game: &Game) {
        set_camera(&self.camera_office);
        clear_background(WHITE);
        draw_texture_ex(
            &self.office_texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                source: None,
                rotation: 0.0,
                dest_size: Some(Vec2::new(1280.0, 720.0)),
                ..Default::default()
            },
        );
        for e in game.get_office().iter_employees() {
            draw_texture_ex(
                &self.employee_texture,
                e.get_pos().x - EMPLOYEE_RADIUS,
                e.get_pos().y - EMPLOYEE_RADIUS,
                WHITE,
                DrawTextureParams {
                    source: None,
                    rotation: e.get_rotation(),
                    dest_size: Some(Vec2::new(500.0, 500.0)),
                    ..Default::default()
                },
            );
        }
    }

    fn draw_info(&self) {
        set_camera(&self.camera_info);
        clear_background(WHITE);
    }

    fn draw_personnal_stat(&mut self, game: &Game) {
        set_camera(&self.camera_personnal_stat);
        clear_background(WHITE);

        let bar_width_max: f32 = 1000.;

        if let Some(selected_employee) = game.get_office().get_selected_employee() {
            draw_rectangle(100., 100., bar_width_max, 100., LIGHTGRAY);
            draw_rectangle(100., 300., bar_width_max, 100., LIGHTGRAY);
            draw_rectangle(100., 500., bar_width_max, 100., LIGHTGRAY);
            draw_rectangle(100., 700., bar_width_max, 100., LIGHTGRAY);

            self.displayed_satisfaction = lerp(
                self.displayed_satisfaction,
                selected_employee.as_ref().borrow().get_satisfaction() * bar_width_max,
                ANIMATION_SPEED,
            );

            self.displayed_energy = lerp(
                self.displayed_energy,
                selected_employee.as_ref().borrow().get_energy() * bar_width_max,
                ANIMATION_SPEED,
            );

            self.displayed_hope = lerp(
                self.displayed_hope,
                selected_employee.as_ref().borrow().get_hope() * bar_width_max,
                ANIMATION_SPEED,
            );

            self.displayed_satiety = lerp(
                self.displayed_satisfaction,
                selected_employee.as_ref().borrow().get_satiety() * bar_width_max,
                ANIMATION_SPEED,
            );

            draw_rectangle(100., 100., self.displayed_satisfaction, 100., RED);

            draw_rectangle(100., 300., self.displayed_energy, 100., YELLOW);

            draw_rectangle(100., 500., self.displayed_satiety, 100., BLUE);

            draw_rectangle(100., 700., self.displayed_hope, 100., GREEN);

            draw_rectangle(
                self.button_personnal_satiety.x,
                self.button_personnal_satiety.y,
                self.button_personnal_satiety.w,
                self.button_personnal_satiety.h,
                GREEN,
            );
            draw_rectangle(
                self.button_personnal_energy.x,
                self.button_personnal_energy.y,
                self.button_personnal_energy.w,
                self.button_personnal_energy.h,
                GREEN,
            );
            draw_rectangle(
                self.button_personnal_satisfaction.x,
                self.button_personnal_satisfaction.y,
                self.button_personnal_satisfaction.w,
                self.button_personnal_satisfaction.h,
                GREEN,
            );
            draw_rectangle(
                self.button_personnal_hope.x,
                self.button_personnal_hope.y,
                self.button_personnal_hope.w,
                self.button_personnal_hope.h,
                GREEN,
            );
        }
    }

    fn draw_global_stat(&self, game: &Game) {
        set_camera(&self.camera_global_stat);
        clear_background(WHITE);
        draw_text(
            &format!(
                "Employees : {}",
                game.get_office().employees_count().to_string()
            ),
            50.,
            100.,
            100.,
            BLACK,
        );
        draw_text(
            &format!("Money : {}", game.get_office().get_money()),
            50.,
            200.,
            100.,
            BLACK,
        )
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

    pub fn draw(&mut self, game: &Game) {
        self.draw_global_stat(game);
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

    pub fn convert_screen_main(coords: Vec2) -> Vec2 {
        let height = screen_width() / 16. * 9.;
        let y = (screen_height() - height) / 2.;

        vec2(
            ((coords.x - 0.) / screen_width()) * GAME_WINDOW_WIDTH as f32,
            ((coords.y - y) / height) * GAME_WINDOW_HEIGHT as f32,
        )
    }

    pub fn convert_main_office(main_coords: Vec2) -> Vec2 {
        let x = GAME_WINDOW_WIDTH as f32 * 0.3;
        let width = GAME_WINDOW_WIDTH as f32 * 0.69;

        let y = GAME_WINDOW_HEIGHT as f32 * 0.3;
        let height = GAME_WINDOW_HEIGHT as f32 * 0.69;

        vec2(
            ((main_coords.x - x) / width) * OFFICE_WIDTH as f32,
            ((main_coords.y - y) / height) * OFFICE_HEIGHT as f32,
        )
    }

    pub fn convert_main_info(&self, main_coords: Vec2) -> Vec2 {
        let x = GAME_WINDOW_WIDTH as f32 * 0.01;
        let width = GAME_WINDOW_WIDTH as f32 * 0.28;

        let y = GAME_WINDOW_HEIGHT as f32 * 0.3;
        let height = GAME_WINDOW_HEIGHT as f32 * 0.69;

        vec2(
            ((main_coords.x - x) / width) * INFO_WIDTH as f32,
            ((main_coords.y - y) / height) * INFO_HEIGHT as f32,
        )
    }

    pub fn convert_main_personnal_stat(&self, main_coords: Vec2) -> Vec2 {
        let x = GAME_WINDOW_WIDTH as f32 * 0.01;
        let width = GAME_WINDOW_WIDTH as f32 * 0.28;

        let y = GAME_WINDOW_HEIGHT as f32 * 0.01;
        let height = GAME_WINDOW_HEIGHT as f32 * 0.28;

        vec2(
            ((main_coords.x - x) / width) * PERSONNAL_STAT_WIDTH as f32,
            ((main_coords.y - y) / height) * PERSONNAL_STAT_HEIGHT as f32,
        )
    }

    pub fn convert_main_global_stat(&self, main_coords: Vec2) -> Vec2 {
        let x = GAME_WINDOW_WIDTH as f32 * 0.3;
        let width = GAME_WINDOW_WIDTH as f32 * 0.69;

        let y = GAME_WINDOW_HEIGHT as f32 * 0.01;
        let height = GAME_WINDOW_HEIGHT as f32 * 0.28;

        vec2(
            ((main_coords.x - x) / width) * GLOBAL_STAT_WIDTH as f32,
            ((main_coords.y - y) / height) * GLOBAL_STAT_HEIGHT as f32,
        )
    }

    pub fn reset_displayed(&mut self) {
        self.displayed_energy = 0.;
        self.displayed_hope = 0.;
        self.displayed_satiety = 0.;
        self.displayed_satisfaction = 0.;
    }
}
