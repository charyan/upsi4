#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_precision_loss,
    clippy::too_many_arguments,
    clippy::cast_possible_truncation
)]

mod assets;
mod drawing;
mod employee;
mod qte;

use drawing::Drawing;
use employee::{EmployeeAction, Office};
use macroquad::{experimental::coroutines::wait_seconds, prelude::*};
use qte::{QteEffect, QTE};

const MIN_PERIOD_WITHOUT_QTE: f32 = 4.;
const MAX_PERIOD_WITHOUT_QTE: f32 = 6.;

const DISPLAY_ANSWER_TIME: f32 = 2.5;

struct Game {
    office: Office,
    qte_ongoing: Option<QTE>,
    starting_time_qte: f32,
    waiting_time_qte: f32,
    next_time_qte: f32,
    answer: Option<String>,
    starting_time_answer: f32,
    qtes: Vec<QTE>,
}

impl Game {
    pub fn new() -> Self {
        let mut new_office = Office::new();
        new_office.add_employee();
        let qtes = vec![
            QTE::new(
                "Voulez vous tuer\nun employée ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., 0., -1),
                QteEffect::new(0., 0., 0., 0., 1., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Il est mort".to_owned(),
                "Il est pas mort".to_owned(),
                3.,
            ),
            QTE::new(
                "Voulez vous perdre\nde l'argent ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., -10000., 0),
                QteEffect::new(0., 0., 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vous êtes con".to_owned(),
                "Bravo".to_owned(),
                2.,
            ),
            QTE::new(
                "Voulez vous recevoir\nun super bonus ?".to_owned(),
                QteEffect::new(1., 1., 1., 1., 0., 0),
                QteEffect::new(0., 0., 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Bravo".to_owned(),
                "Tant pis".to_owned(),
                6.,
            ),
            QTE::new(
                "Voulez vous ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., 0., 0),
                QteEffect::new(0., 0., 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Ah".to_owned(),
                "Oh".to_owned(),
                1.,
            ),
        ];

        Self {
            office: new_office,
            qte_ongoing: None,
            starting_time_qte: 0.,
            waiting_time_qte: 0.,
            starting_time_answer: 0.,
            next_time_qte: MAX_PERIOD_WITHOUT_QTE,
            answer: None,
            qtes,
        }
    }

    pub fn tick(&mut self) {
        self.office.tick();

        if let Some(qte) = &self.qte_ongoing {
            self.waiting_time_qte = 0.;
            if get_time() as f32 - self.starting_time_qte > qte.get_time() {
                self.office.apply_qte_effect(qte.get_effect_1());
                self.quit_qte(qte.get_explication1().to_string());
            }
        } else if let Some(_) = &self.answer {
            if get_time() as f32 - self.starting_time_answer > DISPLAY_ANSWER_TIME {
                self.answer = None;
            }
        } else {
            self.waiting_time_qte += 1.0 / FPS;
        }

        if self.waiting_time_qte > self.next_time_qte {
            self.qte_ongoing = self.launch_qte();
        }
    }

    pub fn get_qte_ongoing(&self) -> &Option<QTE> {
        &self.qte_ongoing
    }

    pub fn get_office(&self) -> &Office {
        &self.office
    }

    pub fn get_mut_office(&mut self) -> &mut Office {
        &mut self.office
    }

    pub fn get_answer(&self) -> &Option<String> {
        &self.answer
    }

    pub fn launch_qte(&mut self) -> Option<QTE> {
        self.starting_time_qte = get_time() as f32;
        let choosed = rand::gen_range(0, self.qtes.len() - 1);

        Some(self.qtes[choosed].clone())
    }

    pub fn quit_qte(&mut self, answer: String) {
        self.qte_ongoing = None;
        self.answer = Some(answer);
        self.starting_time_answer = get_time() as f32;
        self.next_time_qte = rand::gen_range(MIN_PERIOD_WITHOUT_QTE, MAX_PERIOD_WITHOUT_QTE);
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

    loop {
        let start_time = get_time();

        if is_mouse_button_pressed(MouseButton::Left) {
            let main_pos =
                Drawing::convert_screen_main(vec2(mouse_position().0, mouse_position().1));

            if drawing.get_rect_office().contains(main_pos) {
                let pos = Drawing::convert_main_office(main_pos);
                game.get_mut_office().click(pos);
                drawing.reset_displayed();

                println!("Office pos : {:?}", pos);
            } else if drawing.get_rect_info().contains(main_pos) {
                let pos = Drawing::convert_main_info(main_pos);
                println!("Info pos : {:?}", pos);

                if let Some(qte) = game.get_qte_ongoing().clone() {
                    if drawing.get_button_choice_1().contains(pos) {
                        game.get_mut_office().apply_qte_effect(qte.get_effect_1());
                        game.quit_qte(qte.get_explication1().to_string());
                    } else if drawing.get_button_choice_2().contains(pos) {
                        game.get_mut_office().apply_qte_effect(qte.get_effect_2());
                        game.quit_qte(qte.get_explication2().to_string());
                    }
                }
            } else if drawing.get_rect_global_stat().contains(main_pos) {
                let pos = Drawing::convert_main_global_stat(main_pos);
                println!("Drawing pos : {:?}", pos);

                if drawing.get_button_door().contains(pos) {
                    game.get_mut_office().update_door();
                } else if drawing.get_button_meth().contains(pos) {
                    game.get_mut_office().bonus_meth();
                } else if drawing.get_button_rh().contains(pos) {
                    game.get_mut_office().bonus_rh();
                }
            } else if drawing.get_rect_personnal_stat().contains(main_pos) {
                let pos = Drawing::convert_main_personnal_stat(main_pos);
                println!("personnal pos : {:?}", pos);

                if let Some(employee) = game.get_office().get_selected_employee() {
                    let mut employee = employee.as_ref().borrow_mut();

                    match employee.get_state() {
                        employee::EmployeeState::Alive => {
                            if drawing.get_button_energy().contains(pos) {
                                if let EmployeeAction::Sleep = employee.action {
                                    employee.action = EmployeeAction::None;
                                } else {
                                    employee.action = EmployeeAction::Sleep;
                                }
                            } else if drawing.get_button_hope().contains(pos) {
                                if let EmployeeAction::FamilyCall = employee.action {
                                    employee.action = EmployeeAction::None;
                                } else {
                                    employee.action = EmployeeAction::FamilyCall;
                                }
                            } else if drawing.get_button_satiety().contains(pos) {
                                if let EmployeeAction::Eat = employee.action {
                                    employee.action = EmployeeAction::None;
                                } else {
                                    employee.action = EmployeeAction::Eat;
                                }
                            } else if drawing.get_button_satisfaction().contains(pos) {
                                if let EmployeeAction::Break = employee.action {
                                    employee.action = EmployeeAction::None;
                                } else {
                                    employee.action = EmployeeAction::Break;
                                }
                            }
                        }
                        employee::EmployeeState::Dead => {
                            if drawing.get_button_satisfaction().contains(pos) {
                                employee.clean();
                            }
                        }
                        employee::EmployeeState::Falling | employee::EmployeeState::Clean => (),
                        employee::EmployeeState::Suicide => (),
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            game.get_mut_office().add_employee();
        }

        if is_key_pressed(KeyCode::W) {
            game.get_mut_office().update_window();
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
