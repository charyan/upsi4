use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{
    assets,
    employee::{
        DoorState, EmployeeAction, EmployeeState, EMPLOYEE_RADIUS, EMPLOYEE_RUNNING_SPEED,
        EMPLOYEE_SPEED, MIDDLE_LANE,
    },
    Game, DOOR_CD, METH_CD,
};

#[derive(Debug)]
struct RandomPassing {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub speed_factor: f32,
}

const MIN_PERIOD_WITHOUT_PASSING: f32 = 2.;
const MAX_PERIOD_WITHOUT_PASSING: f32 = 3.;

const _TRANSPARENT: Color = Color::new(255., 255., 255., 0.);
const LIGHTGRAY_ALPHA: Color = Color::new(0.78, 0.78, 0.78, 0.4);

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
const FONT_SIZE_PERSONNAL: f32 = 100.;
const FONT_SIZE_BAR: u16 = 75;

const PERSONNAL_LINES_THICKNES: f32 = 35.;

const DESCRIPTION_BUTTON_HOPE: &str = "Laissez votre employée faire un appel vidéo avec sa famille. Mais attention ! Les relations sociales ne participe pas à l'avancement du projet.";
const DESCRIPTION_BUTTON_ENERGY: &str =
    "Laissez votre employée dormir. Mais attention ! Dormir est une perte de temps.";
const DESCRIPTION_BUTTON_SATISFACTION: &str = "Laissez votre employée faire une pause. Mais attention ! Les pauses ne sont absolument pas nécessaire à l'avancement du projet.";
const DESCRIPTION_BUTTON_SATIETY: &str = "Laissez votre employée manger. Mais attention ! Seule la nourriture spirituelle qu'est le travail devrait leur suffire.";

const DESCRIPTION_BUTTON_CLEAN: &str = "Libérez de l'espace en vous débarassant de votre employé";

const DESCRIPTION_BUTTON_DOOR: &str =
    "Ouvrez la porte à vos employée. Plus la porte est ouverte plus ils auront d'espoir.";
const DESCRIPTION_BUTTON_METH: &str =
    "Donnez un coup de boost à vos employée en leur offrant un breuvage (arrangé par vos soin). Coût : 1000";
const DESCRIPTION_BUTTON_RH: &str = "Le pôle RH se démenera afin de vous trouvez LE candidat pour remplir vos rang (probablement un stagiaire). Coût : 200";

const DESCRIPTION_HOPE: &str = "L'espoir de vos employé reflète leurs pensé quand à leur avenir chez vous (pouvoir partir). Trop d'espoir pourrait conduire à une tentive de fuite, alors que pas assez pourrait être facheux. La porte ouverte augmente l'espoir et inversement.";
const DESCRIPTION_SATISFACTION: &str =
    "La joie de vos employé reflète leur bonheur (inefficacité). Des employés trop heureust discutes avec des collègues, baissant drastiquement leur rendement, là où l'inverse pourrait conduire à un départ précipité";
const DESCRIPTION_SATIETY: &str = "La sasiété de vos employé reflète leurs besoin en nourritue. Si vous ne les nourrisez pas, ils risquent de mourir, alors qu'une fois qu'ils ont trop mangé, leur énergie descent drastiquement.";
const DESCRIPTION_ENERGY: &str = "L'énergie de vos employé reflète leur capacité à travailler. Un manque d'énergie conduit à une sieste non-contrôlé alors que trop d'énergie peut avoir des répercussions sur le matériel.";

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start.mul_add(1.0 - t, end * t)
}

pub struct Drawing {
    // Render_target
    pub main_render_target: RenderTarget,
    render_target_office: RenderTarget,
    render_target_info: RenderTarget,
    render_target_global_stat: RenderTarget,
    render_target_personnal_stat: RenderTarget,

    // Camera
    pub main_camera: Camera2D,
    pub camera_office: Camera2D,
    pub camera_info: Camera2D,
    pub camera_global_stat: Camera2D,
    pub camera_personnal_stat: Camera2D,

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

    random_passing: Option<RandomPassing>,
    start_wainting_passing: f32,
    wainting_passing_time: f32,
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
            button_personnal_satisfaction: Rect::new(1325., 135., 250., 150.),
            button_personnal_energy: Rect::new(1325., 335., 250., 150.),
            button_personnal_satiety: Rect::new(1325., 535., 250., 150.),
            button_personnal_hope: Rect::new(1325., 735., 250., 150.),

            // Button global
            button_global_door: Rect::new(120., 100., 125., 125.),
            button_global_meth: Rect::new(270., 100., 125., 125.),
            button_global_rh: Rect::new(420., 100., 125., 125.),

            // Button qte
            button_choice_1: Rect::new(300., 1950., 450., 200.),
            button_choice_2: Rect::new(1150., 1950., 450., 200.),

            //Bar stats
            bar_satisfaction: Rect::new(300., 160., 1000., 100.),
            bar_energy: Rect::new(300., 360., 1000., 100.),
            bar_satiety: Rect::new(300., 560., 1000., 100.),
            bar_hope: Rect::new(300., 760., 1000., 100.),

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

            random_passing: None,
            start_wainting_passing: get_time() as f32,
            wainting_passing_time: MIN_PERIOD_WITHOUT_PASSING,
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

    pub fn handle_passer(&mut self) {
        if let Some(passing) = &mut self.random_passing {
            draw_texture_ex(
                &passing.texture,
                passing.x - EMPLOYEE_RADIUS,
                passing.y - EMPLOYEE_RADIUS,
                WHITE,
                DrawTextureParams {
                    rotation: -PI / 2. * passing.speed_factor,
                    dest_size: Some(Vec2::new(100.0, 100.0)),
                    ..Default::default()
                },
            );
            if passing.y < -15. || passing.y > 750. {
                self.random_passing = None;
                self.start_wainting_passing = get_time() as f32;
                self.wainting_passing_time =
                    rand::gen_range(MIN_PERIOD_WITHOUT_PASSING, MAX_PERIOD_WITHOUT_PASSING);
            } else {
                if passing.y < 424. && passing.y > 290. {
                    passing.y -= EMPLOYEE_RUNNING_SPEED * passing.speed_factor;
                } else {
                    passing.y -= EMPLOYEE_SPEED * passing.speed_factor;
                }
            }
        } else if get_time() as f32 - self.start_wainting_passing > self.wainting_passing_time
            && self.start_wainting_passing != 0.
        {
            let bas = rand::gen_range(0, 2);
            let passer = RandomPassing {
                x: 150.,
                y: if bas == 1 { 750. } else { -15. },
                texture: if rand::gen_range(0, 2) == 1 {
                    assets::EMPLOYEE_TEXTURE.clone()
                } else {
                    assets::MANAGER_TEXTURE.clone()
                },
                speed_factor: if bas == 1 { 1. } else { -1. },
            };
            self.random_passing = Some(passer);
            self.start_wainting_passing = 0.;
        }
    }

    pub fn draw_office_full(&mut self, game: &Game) {
        set_camera(&self.main_camera);

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

        if let DoorState::Closed = game.get_office().get_door_state() {
            if self.door_rotation < 0.0 {
                self.door_rotation += DOOR_SPEED
            } else {
                self.door_rotation = 0.0
            }
        } else if let DoorState::Open = game.get_office().get_door_state() {
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
            if !game.get_office().window_is_open() {
                0.
            } else {
                PI / 2.
            },
            WINDOW_SPEED,
        );

        draw_texture_ex(
            &assets::WINDOW_TEXTURE,
            1070.,
            290.,
            WHITE,
            DrawTextureParams {
                rotation: -self.window_rotation,
                dest_size: Some(vec2(13., 70.)),
                pivot: Some(vec2(1076.5, 290.)),
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
                dest_size: Some(vec2(13., 70.)),
                pivot: Some(vec2(1076.5, 430.)),
                ..Default::default()
            },
        );

        // Draw the computers
        for c in game.get_office().iter_computers_mut() {
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
        self.handle_passer();

        // Draw employees
        for mut e in game.get_office().iter_employees_mut() {
            draw_texture_ex(
                &assets::EMPLOYEE_TEXTURE,
                e.get_pos().x - EMPLOYEE_RADIUS,
                e.get_pos().y - EMPLOYEE_RADIUS,
                WHITE,
                DrawTextureParams {
                    rotation: e.get_rotation(),
                    dest_size: if let EmployeeState::Falling = e.get_state() {
                        let scale = 100.0
                            - (e.get_pos().y - MIDDLE_LANE) / (OFFICE_HEIGHT as f32 - MIDDLE_LANE)
                                * 100.
                            + 50.;
                        Some(Vec2::new(scale, scale))
                    } else {
                        Some(Vec2::new(100.0, 100.0))
                    },
                    ..Default::default()
                },
            );

            let x = e.get_pos().x;
            let y = e.get_pos().y;

            let pos = vec2(x, y);

            if e.get_satiety() < 0.2 {
                e.hungry_emitter.draw(pos);
            } else if e.get_satiety() > 0.8 {
                e.z_emitter.draw(pos);
            }

            if e.get_energy() < 0.2 {
                e.z_emitter.draw(pos);
            } else if e.get_energy() > 0.8 {
                e.lightning_emitter.draw(pos);
            }

            if (e.get_hope() < 0.2) || matches!(e.state, EmployeeState::Suicide) {
                e.cry_emitter.draw(pos);
            } else if e.get_hope() > 0.8 {
                e.heart_emitter.draw(pos);
            }

            if e.get_satisfaction() < 0.2 {
                e.mad1_emitter.draw(pos);
                e.mad2_emitter.draw(pos);
            } else if e.get_satisfaction() > 0.8 {
                e.happy_emitter.draw(pos);
            }
        }

        // Draw fire on broken computers
        game.get_office()
            .iter_computers_mut()
            .filter(|c| c.borrow().broken)
            .for_each(|c| {
                let mut computer = c.borrow_mut();
                let pos = computer.position;
                computer.emitter.draw(pos);
            });
    }

    pub fn draw_office(&mut self, game: &Game) {
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

        if let DoorState::Closed = game.get_office().get_door_state() {
            if self.door_rotation < 0.0 {
                self.door_rotation += DOOR_SPEED
            } else {
                self.door_rotation = 0.0
            }
        } else if let DoorState::Open = game.get_office().get_door_state() {
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
            if !game.get_office().window_is_open() {
                0.
            } else {
                PI / 2.
            },
            WINDOW_SPEED,
        );

        draw_texture_ex(
            &assets::WINDOW_TEXTURE,
            1070.,
            290.,
            WHITE,
            DrawTextureParams {
                rotation: -self.window_rotation,
                dest_size: Some(vec2(13., 70.)),
                pivot: Some(vec2(1076.5, 290.)),
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
                dest_size: Some(vec2(13., 70.)),
                pivot: Some(vec2(1076.5, 430.)),
                ..Default::default()
            },
        );

        // Draw the computers
        for c in game.get_office().iter_computers_mut() {
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

        self.handle_passer();

        // Draw employees
        for mut e in game.get_office().iter_employees_mut() {
            draw_texture_ex(
                &assets::EMPLOYEE_TEXTURE,
                e.get_pos().x - EMPLOYEE_RADIUS,
                e.get_pos().y - EMPLOYEE_RADIUS,
                WHITE,
                DrawTextureParams {
                    rotation: e.get_rotation(),
                    dest_size: if let EmployeeState::Falling = e.get_state() {
                        let scale = 100.0
                            - (e.get_pos().y - MIDDLE_LANE) / (OFFICE_HEIGHT as f32 - MIDDLE_LANE)
                                * 100.
                            + 50.;
                        Some(Vec2::new(scale, scale))
                    } else {
                        Some(Vec2::new(100.0, 100.0))
                    },
                    ..Default::default()
                },
            );

            let x = e.get_pos().x;
            let y = e.get_pos().y;

            let pos = vec2(x, y);

            if matches!(e.get_state(), EmployeeState::Dead) {
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

            if e.get_satiety() < 0.2 {
                e.hungry_emitter.draw(pos);
            } else if e.get_satiety() > 0.8 {
                e.z_emitter.draw(pos);
            }

            if e.get_energy() < 0.2 {
                e.z_emitter.draw(pos);
            } else if e.get_energy() > 0.8 {
                e.lightning_emitter.draw(pos);
            }

            if e.get_hope() < 0.2 {
                e.cry_emitter.draw(pos);
            } else if e.get_hope() > 0.8 {
                e.heart_emitter.draw(pos);
            }

            if e.get_satisfaction() < 0.2 {
                e.mad1_emitter.draw(pos);
                e.mad2_emitter.draw(pos);
            } else if e.get_satisfaction() > 0.8 {
                e.happy_emitter.draw(pos);
            }
        }

        // Draw fire on broken computers
        game.get_office()
            .iter_computers_mut()
            .filter(|c| c.borrow().broken)
            .for_each(|c| {
                let mut computer = c.borrow_mut();
                let pos = computer.position;
                computer.emitter.draw(pos);
            });
    }

    pub fn draw_frame(&self) {
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

    fn draw_info_text(&self, displayed_text: String) {
        let list: Vec<&str> = displayed_text.split(" ").collect();
        let mut new_list: Vec<String> = vec![];

        let mut temp = String::new();
        for word in list.iter() {
            if word.len() + temp.len() < 21 {
                if temp.len() != 0 {
                    temp += " ";
                }
                temp += word;
            } else {
                new_list.push(temp);
                temp = word.to_owned().to_owned();
            }
        }
        new_list.push(temp);

        for (i, text) in new_list.iter().enumerate() {
            draw_text_ex(
                &text,
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

    fn draw_info(&self, game: &Game) {
        set_camera(&self.camera_info);
        clear_background(WHITE);
        let main_pos = Drawing::convert_screen_main(vec2(mouse_position().0, mouse_position().1));

        if let Some(qte) = game.get_qte_ongoing() {
            self.draw_frame_qte();

            self.draw_info_text(qte.get_text().to_owned());

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

            draw_rectangle_lines(
                self.button_choice_1.x,
                self.button_choice_1.y,
                self.button_choice_1.w,
                self.button_choice_1.h,
                35.,
                BLACK,
            );
            draw_rectangle_lines(
                self.button_choice_2.x,
                self.button_choice_2.y,
                self.button_choice_2.w,
                self.button_choice_2.h,
                35.,
                BLACK,
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
            self.draw_frame_qte();
            self.draw_info_text(answer.clone());
        } else {
            if self.rect_personnal_stat.contains(main_pos) {
                if let Some(employee) = game.get_office().get_selected_employee() {
                    match employee.borrow().get_state() {
                        EmployeeState::Alive => {
                            let stat_pos = Self::convert_main_personnal_stat(main_pos);
                            if self.button_personnal_energy.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_BUTTON_ENERGY.to_string());
                            } else if self.button_personnal_hope.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_BUTTON_HOPE.to_string());
                            } else if self.button_personnal_satiety.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_BUTTON_SATIETY.to_string());
                            } else if self.button_personnal_satisfaction.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_BUTTON_SATISFACTION.to_string());
                            } else if self.bar_energy.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_ENERGY.to_string());
                            } else if self.bar_hope.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_HOPE.to_string());
                            } else if self.bar_satiety.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_SATIETY.to_string());
                            } else if self.bar_satisfaction.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_SATISFACTION.to_string());
                            }
                        }
                        EmployeeState::Dead => {
                            let stat_pos = Self::convert_main_personnal_stat(main_pos);
                            if self.button_personnal_satisfaction.contains(stat_pos) {
                                self.draw_frame();
                                self.draw_info_text(DESCRIPTION_BUTTON_CLEAN.to_string());
                            }
                        }
                        EmployeeState::Falling
                        | EmployeeState::Clean
                        | EmployeeState::Suicide
                        | EmployeeState::Arriving => (),
                    }
                }
            }

            if self.rect_global_stat.contains(main_pos) {
                let global_pos = Self::convert_main_global_stat(main_pos);
                if self.button_global_door.contains(global_pos) {
                    self.draw_frame();
                    self.draw_info_text(DESCRIPTION_BUTTON_DOOR.to_string());
                } else if self.button_global_meth.contains(global_pos) {
                    self.draw_frame();
                    self.draw_info_text(DESCRIPTION_BUTTON_METH.to_string());
                } else if self.button_global_rh.contains(global_pos) {
                    self.draw_frame();
                    self.draw_info_text(DESCRIPTION_BUTTON_RH.to_string());
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

    fn draw_bar_name(&self) {
        draw_text_ex(
            "Joie",
            10.,
            self.bar_satisfaction.y + self.bar_satisfaction.h / 2.,
            TextParams {
                font: Some(&assets::FONT),
                font_size: FONT_SIZE_BAR,
                color: BLACK,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Sasiété",
            10.,
            self.bar_satiety.y + self.bar_satiety.h / 2.,
            TextParams {
                font: Some(&assets::FONT),
                font_size: FONT_SIZE_BAR,
                color: BLACK,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Energie",
            10.,
            self.bar_energy.y + self.bar_energy.h / 2.,
            TextParams {
                font: Some(&assets::FONT),
                font_size: FONT_SIZE_BAR,
                color: BLACK,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Espoir",
            10.,
            self.bar_hope.y + self.bar_hope.h / 2.,
            TextParams {
                font: Some(&assets::FONT),
                font_size: FONT_SIZE_BAR,
                color: BLACK,
                ..Default::default()
            },
        );
    }

    fn draw_personnal_stat(&mut self, game: &Game) {
        fn draw_bar(rect: Rect) {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, LIGHTGRAY);
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                PERSONNAL_LINES_THICKNES,
                BLACK,
            );
        }

        set_camera(&self.camera_personnal_stat);
        clear_background(WHITE);

        if let Some(selected_employee) = game.get_office().get_selected_employee() {
            draw_text_ex(
                selected_employee.borrow().get_name(),
                0.,
                FONT_SIZE_PERSONNAL,
                TextParams {
                    font: Some(&assets::FONT),
                    font_size: FONT_SIZE_PERSONNAL as u16,
                    color: BLACK,
                    ..Default::default()
                },
            );
            draw_line(
                5.,
                FONT_SIZE_PERSONNAL + 20.,
                60. * selected_employee.borrow().get_name().len() as f32,
                FONT_SIZE_PERSONNAL + 20.,
                10.,
                BLACK,
            );

            match selected_employee.borrow().get_state() {
                EmployeeState::Dead => {
                    draw_bar(self.bar_satisfaction);
                    draw_bar(self.bar_energy);
                    draw_bar(self.bar_hope);
                    draw_bar(self.bar_satiety);

                    self.draw_bar_name();

                    Drawing::draw_button(
                        self.button_personnal_satisfaction,
                        GREEN,
                        &assets::ICON_CLEAN,
                    );

                    return;
                }
                EmployeeState::Alive => {
                    draw_bar(self.bar_satisfaction);
                    draw_bar(self.bar_energy);
                    draw_bar(self.bar_hope);
                    draw_bar(self.bar_satiety);

                    self.draw_bar_name();

                    self.displayed_satisfaction = lerp(
                        self.displayed_satisfaction,
                        selected_employee.as_ref().borrow().get_satisfaction()
                            * (self.bar_satisfaction.w - PERSONNAL_LINES_THICKNES),
                        ANIMATION_SPEED,
                    );

                    self.displayed_energy = lerp(
                        self.displayed_energy,
                        selected_employee.as_ref().borrow().get_energy()
                            * (self.bar_energy.w - PERSONNAL_LINES_THICKNES),
                        ANIMATION_SPEED,
                    );

                    self.displayed_hope = lerp(
                        self.displayed_hope,
                        selected_employee.as_ref().borrow().get_hope()
                            * (self.bar_hope.w - PERSONNAL_LINES_THICKNES),
                        ANIMATION_SPEED,
                    );

                    self.displayed_satiety = lerp(
                        self.displayed_satiety,
                        selected_employee.as_ref().borrow().get_satiety()
                            * (self.bar_satiety.w - PERSONNAL_LINES_THICKNES),
                        ANIMATION_SPEED,
                    );

                    draw_rectangle(
                        self.bar_satisfaction.x + PERSONNAL_LINES_THICKNES / 2.,
                        self.bar_satisfaction.y + PERSONNAL_LINES_THICKNES / 2.,
                        self.displayed_satisfaction,
                        self.bar_satisfaction.h - PERSONNAL_LINES_THICKNES,
                        RED,
                    );

                    draw_rectangle(
                        self.bar_energy.x + PERSONNAL_LINES_THICKNES / 2.,
                        self.bar_energy.y + PERSONNAL_LINES_THICKNES / 2.,
                        self.displayed_energy,
                        self.bar_energy.h - PERSONNAL_LINES_THICKNES,
                        YELLOW,
                    );

                    draw_rectangle(
                        self.bar_satiety.x + PERSONNAL_LINES_THICKNES / 2.,
                        self.bar_satiety.y + PERSONNAL_LINES_THICKNES / 2.,
                        self.displayed_satiety,
                        self.bar_satiety.h - PERSONNAL_LINES_THICKNES,
                        BLUE,
                    );

                    draw_rectangle(
                        self.bar_hope.x + PERSONNAL_LINES_THICKNES / 2.,
                        self.bar_hope.y + PERSONNAL_LINES_THICKNES / 2.,
                        self.displayed_hope,
                        self.bar_hope.h - PERSONNAL_LINES_THICKNES,
                        GREEN,
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
                        "En chute libre",
                        100.,
                        300.,
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
                        300.,
                        TextParams {
                            font: Some(&assets::FONT),
                            font_size: 100 as u16,
                            color: BLACK,
                            ..Default::default()
                        },
                    );
                }
                EmployeeState::Suicide => {
                    draw_text_ex(
                        "Va se défenestrer",
                        100.,
                        300.,
                        TextParams {
                            font: Some(&assets::FONT),
                            font_size: 100 as u16,
                            color: BLACK,
                            ..Default::default()
                        },
                    );
                }
                EmployeeState::Arriving => {
                    draw_text_ex(
                        "Arriving",
                        100.,
                        300.,
                        TextParams {
                            font: Some(&assets::FONT),
                            font_size: 100 as u16,
                            color: BLACK,
                            ..Default::default()
                        },
                    );
                }
            }
        } else {
            draw_text_ex(
                "Sélectionnez un employé pour",
                100.,
                300.,
                TextParams {
                    font: Some(&assets::FONT),
                    font_size: 100 as u16,
                    color: BLACK,
                    ..Default::default()
                },
            );
            draw_text_ex(
                "voir ses informations.",
                100.,
                400.,
                TextParams {
                    font: Some(&assets::FONT),
                    font_size: 100 as u16,
                    color: BLACK,
                    ..Default::default()
                },
            );
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
            &format!("Money : {}", game.get_office().get_money().round()),
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

        if game.get_start_door_cd() != 0. {
            let value = self.button_global_door.h as f64
                - (get_time() - game.get_start_door_cd()) / DOOR_CD
                    * self.button_global_door.h as f64;

            draw_rectangle(
                self.button_global_door.x,
                self.button_global_door.y + self.button_global_door.h,
                self.button_global_door.w,
                -value as f32,
                LIGHTGRAY_ALPHA,
            );
        }

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
        if game.get_start_meth_cd() != 0. {
            let value = self.button_global_meth.h as f64
                - (get_time() - game.get_start_meth_cd()) / METH_CD
                    * self.button_global_meth.h as f64;

            draw_rectangle(
                self.button_global_meth.x,
                self.button_global_meth.y + self.button_global_meth.h,
                self.button_global_meth.w,
                -value as f32,
                LIGHTGRAY_ALPHA,
            );
        }

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
        if game.get_start_rh_cd() != 0. {
            let value = self.button_global_rh.h as f64
                - (get_time() - game.get_start_rh_cd()) / METH_CD * self.button_global_rh.h as f64;

            draw_rectangle(
                self.button_global_rh.x,
                self.button_global_rh.y + self.button_global_rh.h,
                self.button_global_rh.w,
                -value as f32,
                LIGHTGRAY_ALPHA,
            );
        }
    }

    pub fn draw_game(&self) {
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

    pub fn draw_game_menu(&self) {
        set_camera(&self.main_camera);

        draw_texture_ex(
            &self.render_target_office.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
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

    pub fn draw_menu(&mut self, game: &Game) {
        self.draw_office_full(game);
        self.draw_game_menu();

        set_default_camera();
        clear_background(BLACK);

        let height = screen_width() / 16. * 9.;

        draw_texture_ex(
            &self.main_render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
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
