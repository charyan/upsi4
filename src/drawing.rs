use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{
    assets,
    employee::{DoorState, EmployeeAction, EmployeeState, EMPLOYEE_RADIUS},
    Game,
};

const _TRANSPARENT: Color = Color::new(255., 255., 255., 0.);

pub const OFFICE_WIDTH: u32 = 1280;
pub const OFFICE_HEIGHT: u32 = 720;

pub const GAME_WINDOW_WIDTH: u32 = 1280;
pub const GAME_WINDOW_HEIGHT: u32 = 720;

pub const GLOBAL_STAT_WIDTH: u32 = 1120;
pub const GLOBAL_STAT_HEIGHT: u32 = 270;

pub const PERSONNAL_STAT_WIDTH: u32 = 1600;
pub const PERSONNAL_STAT_HEIGHT: u32 = 900;

pub const INFO_WIDTH: u32 = 1890;
pub const INFO_HEIGHT: u32 = 2480;

pub const ANIMATION_SPEED: f32 = 0.1;
const DOOR_SPEED: f32 = 0.3;
const WINDOW_SPEED: f32 = 0.05;

const FONT_SIZE_INFO: f32 = 150.;
const FONT_SIZE_GLOBAL: f32 = 50.;

const DESCRIPTION_BUTTON_HOPE: &str = "Laissez votre employée\nfaire un appel vidéo\navec sa famille.\n\nMais attention !\nLes relations sociales\nne participe pas\nà l'avancement du \nprojet.";
const DESCRIPTION_BUTTON_ENERGY: &str =
    "Laissez votre employée\ndormir.\n\nMais attention !\nDormir est une\nperte de temps.";
const DESCRIPTION_BUTTON_SATISFACTION: &str = "Laissez votre employée\nfaire une pause.\n\nMais attention !\nLes pauses ne sont\nabsolument pas\nnécessaire à\nl'avancement du projet.";
const DESCRIPTION_BUTTON_SATIETY: &str = "Laissez votre employée\nmanger.\nMais attention !\n\nSeule la nourriture\nspirituelle qu'est le\ntravail devrait\nleur suffire.";

const DESCRIPTION_BUTTON_CLEAN: &str = "Libérez de l'espace en\nvous débarassant de\nvotre employé";

const DESCRIPTION_BUTTON_DOOR: &str = "Ouvrez la porte à vos\nemployée";
const DESCRIPTION_BUTTON_METH: &str = "Donnez un coup de \npouce à vos employée";
const DESCRIPTION_BUTTON_RH: &str = "Recrutez un employée\nvraiment utile";

const DESCRIPTION_HOPE: &str = "Hope";
const DESCRIPTION_SATISFACTION: &str = "Statisfaction";
const DESCRIPTION_SATIETY: &str = "Satiety";
const DESCRIPTION_ENERGY: &str = "Energy";

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
    door_rotation: f32,
    window_rotation: f32,

    // Button personnal
    button_personnal_hope: Rect,
    button_personnal_satiety: Rect,
    button_personnal_energy: Rect,
    button_personnal_satisfaction: Rect,

    // Button global
    button_global_door: Rect,
    button_global_meth: Rect,
    button_global_rh: Rect,

    // Button qte
    button_choice_1: Rect,
    button_choice_2: Rect,

    // Rect render
    rect_office: Rect,
    rect_info: Rect,
    rect_personnal_stat: Rect,
    rect_global_stat: Rect,

    // Bar stat
    bar_satisfaction: Rect,
    bar_energy: Rect,
    bar_satiety: Rect,
    bar_hope: Rect,
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

            // displayed
            displayed_energy: 0.,
            displayed_hope: 0.,
            displayed_satiety: 0.,
            displayed_satisfaction: 0.,
            door_rotation: 0.,
            window_rotation: 0.,

            // Button personnal
            button_personnal_satisfaction: Rect::new(1200., 75., 300., 150.),
            button_personnal_energy: Rect::new(1200., 275., 300., 150.),
            button_personnal_satiety: Rect::new(1200., 475., 300., 150.),
            button_personnal_hope: Rect::new(1200., 675., 300., 150.),

            // Button global
            button_global_door: Rect::new(120., 100., 125., 125.),
            button_global_meth: Rect::new(270., 100., 125., 125.),
            button_global_rh: Rect::new(420., 100., 125., 125.),

            // Button qte
            button_choice_1: Rect::new(300., 1950., 450., 200.),
            button_choice_2: Rect::new(1150., 1950., 450., 200.),

            //Bar stats
            bar_satisfaction: Rect::new(100., 100., 1000., 100.),
            bar_energy: Rect::new(100., 300., 1000., 100.),
            bar_satiety: Rect::new(100., 500., 1000., 100.),
            bar_hope: Rect::new(100., 700., 1000., 100.),

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

    pub fn get_button_rh(&self) -> &Rect {
        &self.button_global_rh
    }

    pub fn get_button_meth(&self) -> &Rect {
        &self.button_global_meth
    }

    pub fn get_button_door(&self) -> &Rect {
        &self.button_global_door
    }

    pub fn get_button_choice_1(&self) -> &Rect {
        &self.button_choice_1
    }

    pub fn get_button_choice_2(&self) -> &Rect {
        &self.button_choice_2
    }

    fn draw_office(&mut self, game: &Game) {
        set_camera(&self.camera_office);
        clear_background(WHITE);
        draw_texture_ex(
            &assets::OFFICE_TEXTURE,
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

        if let DoorState::Open = game.get_office().get_door_state() {
            if self.door_rotation < 0.0 {
                self.door_rotation += DOOR_SPEED
            } else {
                self.door_rotation = 0.0
            }
        } else if let DoorState::Closed = game.get_office().get_door_state() {
            if self.door_rotation > -PI / 2. {
                self.door_rotation -= DOOR_SPEED
            } else {
                self.door_rotation = -PI / 2.
            }
        }

        draw_texture_ex(
            &assets::DOOR_TEXTURE,
            340.,
            320.,
            WHITE,
            DrawTextureParams {
                rotation: self.door_rotation,
                dest_size: Some(vec2(16., 85.)),
                pivot: Some(vec2(348., 400.)),
                ..Default::default()
            },
        );

        self.window_rotation = lerp(
            self.window_rotation,
            if game.get_office().window_is_open() {
                0.
            } else {
                PI / 2.
            },
            WINDOW_SPEED,
        );

        draw_texture_ex(
            &assets::WINDOW_TEXTURE,
            1070.,
            270.,
            WHITE,
            DrawTextureParams {
                rotation: -self.window_rotation,
                dest_size: Some(vec2(16., 85.)),
                pivot: Some(vec2(1078., 270.)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &assets::WINDOW_TEXTURE,
            1070.,
            430.,
            WHITE,
            DrawTextureParams {
                rotation: self.window_rotation + PI,
                dest_size: Some(vec2(16., 85.)),
                pivot: Some(vec2(1078., 430.)),
                ..Default::default()
            },
        );

        // Draw the computers
        for c in game.get_office().iter_computers() {
            let computer = c.borrow();
            let texture: &Texture2D = if computer.broken {
                &assets::COMPUTER_BROKEN_TEXTURE
            } else {
                &assets::COMPUTER_TEXTURE
            };

            draw_texture_ex(
                texture,
                computer.position.x - 25.0,
                computer.position.y - 25.0,
                WHITE,
                DrawTextureParams {
                    rotation: computer.rotation,
                    dest_size: Some(Vec2::new(50.0, 50.0)),
                    ..Default::default()
                },
            );
        }

        for mut e in game.get_office().iter_employees_mut() {
            draw_texture_ex(
                &assets::EMPLOYEE_TEXTURE,
                e.get_pos().x - EMPLOYEE_RADIUS,
                e.get_pos().y - EMPLOYEE_RADIUS,
                WHITE,
                DrawTextureParams {
                    rotation: e.get_rotation(),
                    dest_size: Some(Vec2::new(100.0, 100.0)),
                    ..Default::default()
                },
            );

            match e.get_state() {
                crate::employee::EmployeeState::Dead => {
                    draw_texture_ex(
                        &assets::X_TEXTURE,
                        e.get_pos().x - EMPLOYEE_RADIUS / 2.,
                        e.get_pos().y - EMPLOYEE_RADIUS / 2.,
                        WHITE,
                        DrawTextureParams {
                            rotation: e.get_rotation(),
                            dest_size: Some(Vec2::new(EMPLOYEE_RADIUS, EMPLOYEE_RADIUS)),
                            ..Default::default()
                        },
                    );
                }
                _ => {
                    (match e.action {
                        EmployeeAction::Sleep => {
                            let x = e.get_pos().x;
                            let y = e.get_pos().y;
                            e.emitter.draw(vec2(x, y));
                        }
                        _ => (),
                    });
                }
            }
        }
    }

    fn draw_frame(&self) {
        draw_texture_ex(
            &assets::FRAME_TEXTURE,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                source: None,
                rotation: 0.0,
                dest_size: Some(Vec2::new(INFO_WIDTH as f32, INFO_HEIGHT as f32)),
                ..Default::default()
            },
        )
    }

    fn draw_frame_qte(&self) {
        draw_texture_ex(
            &assets::FRAME_MAGENTA_TEXTURE,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                source: None,
                rotation: 0.0,
                dest_size: Some(Vec2::new(INFO_WIDTH as f32, INFO_HEIGHT as f32)),
                ..Default::default()
            },
        )
    }

    fn draw_info(&self, game: &Game) {
        set_camera(&self.camera_info);
        clear_background(WHITE);
        let main_pos = Drawing::convert_screen_main(vec2(mouse_position().0, mouse_position().1));

        if let Some(qte) = game.get_qte_ongoing() {
            self.draw_frame_qte();

            for (i, text) in qte.get_text().split("\n").enumerate() {
                draw_text_ex(
                    text,
                    FONT_SIZE_INFO + 50.,
                    300. + i as f32 * FONT_SIZE_INFO + 10.,
                    TextParams {
                        font: Some(&assets::FONT),
                        font_size: FONT_SIZE_INFO as u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }

            draw_rectangle(
                self.button_choice_1.x,
                self.button_choice_1.y,
                self.button_choice_1.w,
                self.button_choice_1.h,
                GREEN,
            );
            draw_rectangle(
                self.button_choice_2.x,
                self.button_choice_2.y,
                self.button_choice_2.w,
                self.button_choice_2.h,
                GREEN,
            );

            draw_text_ex(
                qte.get_choice1(),
                (self.button_choice_1.w - FONT_SIZE_INFO * qte.get_choice1().len() as f32) / 2.
                    + self.button_choice_1.x
                    + FONT_SIZE_INFO / 2.,
                self.button_choice_1.y + (self.button_choice_1.h / 2.) - FONT_SIZE_INFO / 2.
                    + FONT_SIZE_INFO,
                TextParams {
                    font: Some(&assets::FONT),
                    font_size: FONT_SIZE_INFO as u16,
                    color: BLACK,
                    ..Default::default()
                },
            );

            draw_text_ex(
                qte.get_choice2(),
                (self.button_choice_2.w - FONT_SIZE_INFO * qte.get_choice2().len() as f32) / 2.
                    + self.button_choice_2.x
                    + FONT_SIZE_INFO / 2.,
                self.button_choice_2.y + (self.button_choice_2.h / 2.) - FONT_SIZE_INFO / 2.
                    + FONT_SIZE_INFO,
                TextParams {
                    font: Some(&assets::FONT),
                    font_size: FONT_SIZE_INFO as u16,
                    color: BLACK,
                    ..Default::default()
                },
            );

            let color = Color::new(
                (get_time() as f32 - game.starting_time_qte) / qte.get_time(),
                0.,
                1. - (get_time() as f32 - game.starting_time_qte) / qte.get_time(),
                1.,
            );

            draw_rectangle(300., 2200., 1350., 100., color);
            draw_rectangle(
                300. + (get_time() as f32 - game.starting_time_qte) / qte.get_time() * 1350.,
                2200.,
                1350. - (get_time() as f32 - game.starting_time_qte) / qte.get_time() * 1350.,
                100.,
                LIGHTGRAY,
            );

            draw_rectangle_lines(300., 2200., 1350., 100., 50., BLACK);
        } else if let Some(answer) = game.get_answer() {
            self.draw_frame();

            for (i, text) in answer.split("\n").enumerate() {
                draw_text_ex(
                    text,
                    FONT_SIZE_INFO + 50.,
                    300. + i as f32 * FONT_SIZE_INFO + 10.,
                    TextParams {
                        font: Some(&assets::FONT),
                        font_size: FONT_SIZE_INFO as u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
        } else {
            if self.rect_personnal_stat.contains(main_pos) {
                if let Some(employee) = game.get_office().get_selected_employee() {
                    match employee.borrow().get_state() {
                        EmployeeState::Alive => {
                            let stat_pos = Self::convert_main_personnal_stat(main_pos);
                            if self.button_personnal_energy.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in DESCRIPTION_BUTTON_ENERGY.split("\n").enumerate() {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            } else if self.button_personnal_hope.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in DESCRIPTION_BUTTON_HOPE.split("\n").enumerate() {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            } else if self.button_personnal_satiety.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in DESCRIPTION_BUTTON_SATIETY.split("\n").enumerate()
                                {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            } else if self.button_personnal_satisfaction.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in
                                    DESCRIPTION_BUTTON_SATISFACTION.split("\n").enumerate()
                                {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            } else if self.bar_energy.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in DESCRIPTION_ENERGY.split("\n").enumerate() {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            } else if self.bar_hope.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in DESCRIPTION_HOPE.split("\n").enumerate() {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            } else if self.bar_satiety.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in DESCRIPTION_SATIETY.split("\n").enumerate() {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            } else if self.bar_satisfaction.contains(stat_pos) {
                                self.draw_frame();

                                for (i, text) in DESCRIPTION_SATISFACTION.split("\n").enumerate() {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            }
                        }
                        EmployeeState::Dead => {
                            let stat_pos = Self::convert_main_personnal_stat(main_pos);
                            if self.button_personnal_satisfaction.contains(stat_pos) {
                                self.draw_frame();
                                for (i, text) in DESCRIPTION_BUTTON_CLEAN.split("\n").enumerate() {
                                    draw_text_ex(
                                        text,
                                        FONT_SIZE_INFO + 50.,
                                        300. + i as f32 * FONT_SIZE_INFO + 10.,
                                        TextParams {
                                            font: Some(&assets::FONT),
                                            font_size: FONT_SIZE_INFO as u16,
                                            color: BLACK,
                                            ..Default::default()
                                        },
                                    );
                                }
                            }
                        }
                        EmployeeState::Falling | EmployeeState::Clean => (),
                    }
                }
            }

            if self.rect_global_stat.contains(main_pos) {
                let global_pos = Self::convert_main_global_stat(main_pos);
                if self.button_global_door.contains(global_pos) {
                    self.draw_frame();

                    for (i, text) in DESCRIPTION_BUTTON_DOOR.split("\n").enumerate() {
                        draw_text_ex(
                            text,
                            FONT_SIZE_INFO + 50.,
                            300. + i as f32 * FONT_SIZE_INFO + 10.,
                            TextParams {
                                font: Some(&assets::FONT),
                                font_size: FONT_SIZE_INFO as u16,
                                color: BLACK,
                                ..Default::default()
                            },
                        );
                    }
                } else if self.button_global_meth.contains(global_pos) {
                    self.draw_frame();

                    for (i, text) in DESCRIPTION_BUTTON_METH.split("\n").enumerate() {
                        draw_text_ex(
                            text,
                            FONT_SIZE_INFO + 50.,
                            300. + i as f32 * FONT_SIZE_INFO + 10.,
                            TextParams {
                                font: Some(&assets::FONT),
                                font_size: FONT_SIZE_INFO as u16,
                                color: BLACK,
                                ..Default::default()
                            },
                        );
                    }
                } else if self.button_global_rh.contains(global_pos) {
                    self.draw_frame();

                    for (i, text) in DESCRIPTION_BUTTON_RH.split("\n").enumerate() {
                        draw_text_ex(
                            text,
                            FONT_SIZE_INFO + 50.,
                            300. + i as f32 * FONT_SIZE_INFO + 10.,
                            TextParams {
                                font: Some(&assets::FONT),
                                font_size: FONT_SIZE_INFO as u16,
                                color: BLACK,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    }

    fn draw_button(rect: Rect, color: Color, texture: &Texture2D) {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect.w, rect.h)),
                ..Default::default()
            },
        );
    }

    fn draw_personnal_stat(&mut self, game: &Game) {
        set_camera(&self.camera_personnal_stat);
        clear_background(WHITE);

        if let Some(selected_employee) = game.get_office().get_selected_employee() {
            match selected_employee.borrow().get_state() {
                EmployeeState::Dead => {
                    draw_rectangle(
                        self.bar_satisfaction.x,
                        self.bar_satisfaction.y,
                        self.bar_satisfaction.w,
                        self.bar_satisfaction.h,
                        LIGHTGRAY,
                    );
                    draw_rectangle(
                        self.bar_energy.x,
                        self.bar_energy.y,
                        self.bar_energy.w,
                        self.bar_energy.h,
                        LIGHTGRAY,
                    );
                    draw_rectangle(
                        self.bar_satiety.x,
                        self.bar_satiety.y,
                        self.bar_satiety.w,
                        self.bar_satiety.h,
                        LIGHTGRAY,
                    );
                    draw_rectangle(
                        self.bar_hope.x,
                        self.bar_hope.y,
                        self.bar_hope.w,
                        self.bar_hope.h,
                        LIGHTGRAY,
                    );

                    Drawing::draw_button(
                        self.button_personnal_satisfaction,
                        GREEN,
                        &assets::ICON_CLEAN,
                    );

                    return;
                }
                EmployeeState::Alive => {
                    draw_rectangle(
                        self.bar_satisfaction.x,
                        self.bar_satisfaction.y,
                        self.bar_satisfaction.w,
                        self.bar_satisfaction.h,
                        LIGHTGRAY,
                    );
                    draw_rectangle(
                        self.bar_energy.x,
                        self.bar_energy.y,
                        self.bar_energy.w,
                        self.bar_energy.h,
                        LIGHTGRAY,
                    );
                    draw_rectangle(
                        self.bar_satiety.x,
                        self.bar_satiety.y,
                        self.bar_satiety.w,
                        self.bar_satiety.h,
                        LIGHTGRAY,
                    );
                    draw_rectangle(
                        self.bar_hope.x,
                        self.bar_hope.y,
                        self.bar_hope.w,
                        self.bar_hope.h,
                        LIGHTGRAY,
                    );

                    self.displayed_satisfaction = lerp(
                        self.displayed_satisfaction,
                        selected_employee.as_ref().borrow().get_satisfaction()
                            * self.bar_satisfaction.w,
                        ANIMATION_SPEED,
                    );

                    self.displayed_energy = lerp(
                        self.displayed_energy,
                        selected_employee.as_ref().borrow().get_energy() * self.bar_energy.w,
                        ANIMATION_SPEED,
                    );

                    self.displayed_hope = lerp(
                        self.displayed_hope,
                        selected_employee.as_ref().borrow().get_hope() * self.bar_hope.w,
                        ANIMATION_SPEED,
                    );

                    self.displayed_satiety = lerp(
                        self.displayed_satiety,
                        selected_employee.as_ref().borrow().get_satiety() * self.bar_satiety.w,
                        ANIMATION_SPEED,
                    );

                    draw_rectangle(100., 100., self.displayed_satisfaction, 100., RED);

                    draw_rectangle(100., 300., self.displayed_energy, 100., YELLOW);

                    draw_rectangle(100., 500., self.displayed_satiety, 100., BLUE);

                    draw_rectangle(100., 700., self.displayed_hope, 100., GREEN);

                    draw_rectangle_lines(
                        self.bar_satisfaction.x,
                        self.bar_satisfaction.y,
                        self.bar_satisfaction.w,
                        self.bar_satisfaction.h,
                        35.,
                        BLACK,
                    );
                    draw_rectangle_lines(
                        self.bar_energy.x,
                        self.bar_energy.y,
                        self.bar_energy.w,
                        self.bar_energy.h,
                        35.,
                        BLACK,
                    );
                    draw_rectangle_lines(
                        self.bar_satiety.x,
                        self.bar_satiety.y,
                        self.bar_satiety.w,
                        self.bar_satiety.h,
                        35.,
                        BLACK,
                    );
                    draw_rectangle_lines(
                        self.bar_hope.x,
                        self.bar_hope.y,
                        self.bar_hope.w,
                        self.bar_hope.h,
                        35.,
                        BLACK,
                    );

                    Drawing::draw_button(
                        self.button_personnal_satiety,
                        if let EmployeeAction::Eat = selected_employee.borrow().action {
                            RED
                        } else {
                            GREEN
                        },
                        &assets::ICON_SATIETY,
                    );

                    Drawing::draw_button(
                        self.button_personnal_energy,
                        if let EmployeeAction::Sleep = selected_employee.borrow().action {
                            RED
                        } else {
                            GREEN
                        },
                        &assets::ICON_ENERGY,
                    );

                    Drawing::draw_button(
                        self.button_personnal_satisfaction,
                        if let EmployeeAction::Break = selected_employee.borrow().action {
                            RED
                        } else {
                            GREEN
                        },
                        &assets::ICON_SATISFACTION,
                    );

                    Drawing::draw_button(
                        self.button_personnal_hope,
                        if let EmployeeAction::FamilyCall = selected_employee.borrow().action {
                            RED
                        } else {
                            GREEN
                        },
                        &assets::ICON_HOPE,
                    );
                }
                EmployeeState::Falling => {
                    draw_text_ex(
                        "Falling",
                        100.,
                        100.,
                        TextParams {
                            font: Some(&assets::FONT),
                            font_size: 100 as u16,
                            color: BLACK,
                            ..Default::default()
                        },
                    );
                }
                EmployeeState::Clean => {
                    draw_text_ex(
                        "Removed",
                        100.,
                        100.,
                        TextParams {
                            font: Some(&assets::FONT),
                            font_size: 100 as u16,
                            color: BLACK,
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    fn draw_global_stat(&self, game: &Game) {
        set_camera(&self.camera_global_stat);
        clear_background(WHITE);
        draw_text_ex(
            &format!(
                "Employees : {}",
                game.get_office().employees_count().to_string()
            ),
            700.,
            50.,
            TextParams {
                font: Some(&assets::FONT),
                font_size: FONT_SIZE_GLOBAL as u16,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_text_ex(
            &format!("Money : {}", game.get_office().get_money()),
            700.,
            100.,
            TextParams {
                font: Some(&assets::FONT),
                font_size: FONT_SIZE_GLOBAL as u16,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_rectangle(
            self.button_global_door.x,
            self.button_global_door.y,
            self.button_global_door.w,
            self.button_global_door.h,
            GREEN,
        );
        draw_texture_ex(
            &assets::ICON_DOOR,
            self.button_global_door.x,
            self.button_global_door.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.button_global_door.w, self.button_global_door.h)),
                ..Default::default()
            },
        );
        draw_rectangle_lines(
            self.button_global_door.x,
            self.button_global_door.y,
            self.button_global_door.w,
            self.button_global_door.h,
            10.,
            BLACK,
        );

        draw_rectangle(
            self.button_global_meth.x,
            self.button_global_meth.y,
            self.button_global_meth.w,
            self.button_global_meth.h,
            GREEN,
        );
        draw_texture_ex(
            &assets::ICON_METH,
            self.button_global_meth.x,
            self.button_global_meth.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.button_global_meth.w, self.button_global_meth.h)),
                ..Default::default()
            },
        );
        draw_rectangle_lines(
            self.button_global_meth.x,
            self.button_global_meth.y,
            self.button_global_meth.w,
            self.button_global_meth.h,
            10.,
            BLACK,
        );

        draw_rectangle(
            self.button_global_rh.x,
            self.button_global_rh.y,
            self.button_global_rh.w,
            self.button_global_rh.h,
            GREEN,
        );
        draw_texture_ex(
            &assets::ICON_RH,
            self.button_global_rh.x,
            self.button_global_rh.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.button_global_rh.w, self.button_global_rh.h)),
                ..Default::default()
            },
        );
        draw_rectangle_lines(
            self.button_global_rh.x,
            self.button_global_rh.y,
            self.button_global_rh.w,
            self.button_global_rh.h,
            10.,
            BLACK,
        );
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
        self.draw_info(game);
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

    pub fn convert_main_info(main_coords: Vec2) -> Vec2 {
        let x = GAME_WINDOW_WIDTH as f32 * 0.01;
        let width = GAME_WINDOW_WIDTH as f32 * 0.28;

        let y = GAME_WINDOW_HEIGHT as f32 * 0.3;
        let height = GAME_WINDOW_HEIGHT as f32 * 0.69;

        vec2(
            ((main_coords.x - x) / width) * INFO_WIDTH as f32,
            ((main_coords.y - y) / height) * INFO_HEIGHT as f32,
        )
    }

    pub fn convert_main_personnal_stat(main_coords: Vec2) -> Vec2 {
        let x = GAME_WINDOW_WIDTH as f32 * 0.01;
        let width = GAME_WINDOW_WIDTH as f32 * 0.28;

        let y = GAME_WINDOW_HEIGHT as f32 * 0.01;
        let height = GAME_WINDOW_HEIGHT as f32 * 0.28;

        vec2(
            ((main_coords.x - x) / width) * PERSONNAL_STAT_WIDTH as f32,
            ((main_coords.y - y) / height) * PERSONNAL_STAT_HEIGHT as f32,
        )
    }

    pub fn convert_main_global_stat(main_coords: Vec2) -> Vec2 {
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
