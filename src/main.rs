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

use std::{cell::RefCell, rc::Rc};

use drawing::Drawing;
use employee::{EmployeeAction, Office, BONUS_METH_COST, BONUS_RH_COST};
use macroquad::{experimental::coroutines::wait_seconds, prelude::*};
use qte::{QteEffect, QTE};

const MIN_PERIOD_WITHOUT_QTE: f32 = 4.;
const MAX_PERIOD_WITHOUT_QTE: f32 = 6.;

const DISPLAY_ANSWER_TIME: f32 = 2.5;

pub const DOOR_CD: f64 = 0.5;
pub const RH_CD: f64 = 3.;
pub const METH_CD: f64 = 5.;

#[derive(Clone, Copy)]
pub enum GameState {
    Running,
    GameOver,
    MyLittleOfficeMenu,
    CrunchSimulatorMenu,
}

struct Game {
    drawing: Rc<RefCell<Drawing>>,
    office: Office,
    qte_ongoing: Option<QTE>,
    starting_time_qte: f32,
    waiting_time_qte: f32,
    next_time_qte: f32,
    answer: Option<String>,
    starting_time_answer: f32,
    qtes: Vec<QTE>,
    game_state: GameState,
    door_start_cd: f64,
    rh_start_cd: f64,
    meth_start_cd: f64,
    menu: Rc<RefCell<Menu>>,
}

impl Game {
    pub fn new() -> Self {
        let drawing = Rc::new(RefCell::new(Drawing::new()));

        let mut new_office = Office::new();
        new_office.add_employee();
        let qtes = vec![
            QTE::new(
                "C'est l'hiver. Il fait froid. Est-ce que les employés ont le droit à du chauffage ?".to_owned(),
                QteEffect::new(-0.3, 0., 0., 0., 0., 0),
                QteEffect::new(0., 0., 0., 0., -200., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Après avoir vu votre employé essayé de travaillé avec des moufles, vous comprenez qu'il fait peut-être trop froid. Les employés vous déteste un peu plus".to_owned(),
                "Le chauffage coûte cher, mais vos employés resteront peut-être plus longtemps.".to_owned(),
                4.,
            ),
            QTE::new(
                "Vos employés ont faim. Faire une pause déjeuner ?".to_owned(),
                QteEffect::new(0., -0.2, 0.2, 0., 0., 0),
                QteEffect::new(0., 0., -0.2, 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vos employés ont pu manger, mais vu la raclette qu'ils ont mangé, ils perdent toute leur énergie".to_owned(),
                "Vous êtes si prévenant de penser à leur ligne !".to_owned(),
                4.,
            ),
            QTE::new(
                "C'est l'été. Il fait chaud. Très chaud. Est-ce que vos employés ont le droit à la clim ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., -200., 0),
                QteEffect::new(0., 0., 0., 0., 200., -1),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vos employés sont au frais, pour le moment. Pour votre porte-monnaie par contre, c'est chaud.".to_owned(),
                "Un employé est mort à cause de la chaleur. Cependant, grâce aux économies, il devrait être possible de faire un nouveau contrat".to_owned(),
                4.,
            ),
            QTE::new(
                "Un stagiaire passe dans le couloir, le capturez ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., 0., 1),
                QteEffect::new(0., 0., 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Après avoir utilisé votre meilleur lasso, le stagiaire fini dans la pièce. A vous de le gérez ! ".to_owned(),
                "Vous l'avez laissé passer".to_owned(),
                2.,
            ),
            QTE::new(
                "C'est Nöel, vos employés demande un jour de vancances... Leur accordé ?".to_owned(),
                QteEffect::new(0.2, 0., 0., 0.4, 0., 0),
                QteEffect::new(-0.2, 0., 0., -0.4, 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vos employés sont remplis d'espoir face à cette nouvelle, ils sont également très satisfait.".to_owned(),
                "Vos employés perdent tout espoir et son mécontent".to_owned(),
                4.,
            ),
            QTE::new(
                "Après 80 semaines intensives, vos employés osent demandé un jour de vacances. Leur accorder ?".to_owned(),
                QteEffect::new(0., 0.1, 0., 0., -100., 0),
                QteEffect::new(0., -0.3, 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vos employé reviennent plus reposé, mais celà à coûté à l'entreprise".to_owned(),
                "Vos employés sont terriblement fatigués.".to_owned(),
                4.,
            ),
            QTE::new(
                "Un stagiaire demande à vous voir. Une fois dans votre bureau, il se montre... sugestif quand à ses capacités. L'embaucher ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., 0., 1),
                QteEffect::new(0.2, 0., 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vous passez un très bon moment et de plus, vous venez de gagner un employé fidèle".to_owned(),
                "Vos employés, qui avaient eu vent de l'affaire, apprécie votre comportement".to_owned(),
                4.,
            ),
            QTE::new(
                "Une employée vous menace de vous dénoncer au syndicat. Voulez-vous l'élimiée ?".to_owned(),
                QteEffect::new(0., 0., 0., -0.3, 0., -1),
                QteEffect::new(-0.3, 0., 0., -0.3, 0., -1),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Bien joué ! Des décisions difficile doivent être prise en tant que manager pour la bonne santé de la boite. Vos employés perdent espoir, mais le syndicat ne sera pas au courant. ".to_owned(),
                "Malheureusement, votre big boss condamne votre inaction et se charge lui même d'élimier la menace. Mais puisque qu'il n'a pas votre expérience dans le métier, le résultat est brouillon et vos employés l'apprennent.".to_owned(),
                4.,
            ),
            QTE::new(
                "Vous suprenez un employé en train de se détendre pendant sa pause en regardant internet. Installer un firewall afin de bloquer tous les sites de distraction ?".to_owned(),
                QteEffect::new(-0.3, 0.2, 0., 0., -100., 0),
                QteEffect::new(0.3, 0., 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vos employés sont très insatisfait, mais leur énergie augmente grâce au sevrage que vous leur imposer. Ah ! et votre porte-monnaie en a pris un coup aussi.".to_owned(),
                "Vos employés apprécie ce geste de clémence".to_owned(),
                4.,
            ),
            QTE::new(
                "L'alarme incendie retenti, voulez-vous évacuer vos employés ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., -1000., 0),
                QteEffect::new(0., 0., 0., 0., 0., -2),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vos employés s'en sortent indemne, contrairement à votre porte-monnaie qui subit ce manque à gagner extrême".to_owned(),
                "Le jeu continu d'être développé et avance parfaitement, bien que quelque perte soient à déplorer.".to_owned(),
                4.,
            ),
            QTE::new(
                "Votre big boss demande ou en est le jeu. Lui mentir ?".to_owned(),
                QteEffect::new(-0.1, -0.1, -0.1, -0.1, 0., 0),
                QteEffect::new(-0.3, -0.3, 0., 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Une fois votre mensonge terminé, vous êtes obligé d'augmenter encore la productivité, et vos employés osent se plaindre".to_owned(),
                "Une fois la vérité étalé, votre boss vous ordonne de passer le temps de travail journalier de 20h à 21h".to_owned(),
                4.,
            ),
            QTE::new(
                "Mail urgent ! Ouvrir maintenant ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., -100., 0),
                QteEffect::new(0., 0., 0., 0., 0., -1),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Il ne fallait pas l'ouvrir, il s'agissait d'un virus. Vous achetez un nouveau PC".to_owned(),
                "Bien joué, il s'agissait en réalité d'un virus envoyé par vos employés. Vous décidez donc d'en virez un.".to_owned(),
                1.5,
            ),
            QTE::new(
                "Un enfant de 10 ans propose de travailler pour vous. Ses compétences vous impréssionne. L'engagez ?".to_owned(),
                QteEffect::new(-0.3, 0., 0., -0.3, 0., 1),
                QteEffect::new(0., 0., 0., -0.2, 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Cet enfant est tout à fait compétent, mais les autres employés n'apprécient pas trop de faire travailler un mineur. Il ne sont jamais content".to_owned(),
                "L'enfant repart en pleurant, faisant perdre à vos autres employé tout espoir en vous.".to_owned(),
                4.,
            ),
            QTE::new(
                "Après de nombreux suicide, vos employés propose de baricader la fenêtre. Les écouter ?".to_owned(),
                QteEffect::new(0., 0., 0., 0., -300., 0),
                QteEffect::new(0., 0., 0., -0.3, 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vous payez une entreprise pour baricader cette fenêtre, mais cette dernière ne vient jamais, ayant peur de rester enfermé à jamais.".to_owned(),
                "Les employées perdent espoir quand à leur chance de survie.".to_owned(),
                4.,
            ),
            QTE::new(
                "Le canitinier vient vous voir annonçant que la nourriture est perimé. L'utiliser quand même ?".to_owned(),
                QteEffect::new(0., -0.1, 0., 0., 0., 0),
                QteEffect::new(0., 0., -0.3, 0., 0., 0),
                "Oui".to_owned(),
                "Non".to_owned(),
                "Vos employés tombent malade, et leur énergie en prend un coup".to_owned(),
                "Vos employés ont très faim car il ne mange pas".to_owned(),
                3.,
            ),
        ];

        Self {
            drawing,
            office: new_office,
            qte_ongoing: None,
            starting_time_qte: 0.,
            waiting_time_qte: 0.,
            starting_time_answer: 0.,
            next_time_qte: MAX_PERIOD_WITHOUT_QTE,
            answer: None,
            qtes,
            rh_start_cd: 0.,
            meth_start_cd: 0.,
            door_start_cd: 0.,
            game_state: GameState::MyLittleOfficeMenu, // TODO initial state should be Game menu
            menu: Rc::new(RefCell::new(Menu::new())),
        }
    }

    pub fn in_game_event_handling(&mut self) {
        let drawing_clone = self.drawing.clone();
        let mut drawing = drawing_clone.borrow_mut();

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

        if is_mouse_button_pressed(MouseButton::Left) {
            let main_pos =
                Drawing::convert_screen_main(vec2(mouse_position().0, mouse_position().1));

            if drawing.get_rect_office().contains(main_pos) {
                let pos = Drawing::convert_main_office(main_pos);
                self.get_mut_office().click(pos);
                drawing.reset_displayed();

                println!("Office pos : {:?}", pos);
            } else if drawing.get_rect_info().contains(main_pos) {
                let pos = Drawing::convert_main_info(main_pos);
                println!("Info pos : {:?}", pos);

                if let Some(qte) = self.get_qte_ongoing().clone() {
                    if drawing.get_button_choice_1().contains(pos) {
                        self.get_mut_office().apply_qte_effect(qte.get_effect_1());
                        self.quit_qte(qte.get_explication1().to_string());
                    } else if drawing.get_button_choice_2().contains(pos) {
                        self.get_mut_office().apply_qte_effect(qte.get_effect_2());
                        self.quit_qte(qte.get_explication2().to_string());
                    }
                }
            } else if drawing.get_rect_global_stat().contains(main_pos) {
                let pos = Drawing::convert_main_global_stat(main_pos);

                if drawing.get_button_door().contains(pos) {
                    if self.door_start_cd == 0. {
                        self.get_mut_office().update_door();
                        self.door_start_cd = get_time();
                    }
                } else if drawing.get_button_meth().contains(pos) {
                    if self.meth_start_cd == 0. && self.office.get_money() >= BONUS_METH_COST {
                        self.office
                            .set_money(self.office.get_money() - BONUS_METH_COST);

                        self.get_mut_office().bonus_meth();
                        self.meth_start_cd = get_time();
                    }
                } else if drawing.get_button_rh().contains(pos) {
                    if self.rh_start_cd == 0. && self.office.get_money() >= BONUS_RH_COST {
                        self.office
                            .set_money(self.office.get_money() - BONUS_RH_COST);
                        self.get_mut_office().bonus_rh();
                        self.rh_start_cd = get_time();
                    }
                }
            } else if drawing.get_rect_personnal_stat().contains(main_pos) {
                let pos = Drawing::convert_main_personnal_stat(main_pos);
                println!("personnal pos : {:?}", pos);

                if let Some(employee) = self.get_office().get_selected_employee() {
                    let mut employee = employee.as_ref().borrow_mut();

                    match employee.get_state() {
                        employee::EmployeeState::Alive => {
                            if !matches!(employee.action, EmployeeAction::ForcedSleep) {
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
                        }
                        employee::EmployeeState::Dead => {
                            if drawing.get_button_satisfaction().contains(pos) {
                                employee.clean();
                            }
                        }
                        employee::EmployeeState::Falling
                        | employee::EmployeeState::Clean
                        | employee::EmployeeState::Suicide
                        | employee::EmployeeState::Arriving => (),
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            self.get_mut_office().add_employee();
        }

        if is_key_pressed(KeyCode::W) {
            self.get_mut_office().update_window();
        }
    }

    pub fn tick(&mut self) {
        match self.game_state {
            GameState::Running => {
                self.office.tick();

                if get_time() - self.door_start_cd > DOOR_CD {
                    self.door_start_cd = 0.
                }
                if get_time() - self.meth_start_cd > METH_CD {
                    self.meth_start_cd = 0.
                } else if get_time() - self.rh_start_cd > RH_CD {
                    self.rh_start_cd = 0.
                }

                self.in_game_event_handling();

                self.drawing.clone().borrow_mut().draw(&self);

                if self.office.is_game_over() && self.answer.is_none() {
                    self.game_state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                let menu_clone = self.menu.clone();
                let mut menu = menu_clone.borrow_mut();

                menu.state = MenuState::GameOver;

                menu.draw(self);
                menu.tick(self);
            }
            GameState::MyLittleOfficeMenu => {
                let menu_clone = self.menu.clone();
                let mut menu = menu_clone.borrow_mut();

                menu.draw(self);
                menu.tick(self);

                if menu.game_started {
                    self.game_state = GameState::Running;
                }
            }
            GameState::CrunchSimulatorMenu => (),
        }
    }

    pub fn get_start_door_cd(&self) -> f64 {
        self.door_start_cd
    }

    pub fn get_start_rh_cd(&self) -> f64 {
        self.rh_start_cd
    }

    pub fn get_start_meth_cd(&self) -> f64 {
        self.meth_start_cd
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
        let choosed = rand::gen_range(0, self.qtes.len());

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
    let mut game = Game::new();

    loop {
        let start_time = get_time();

        game.tick();

        next_frame().await;

        let elapsed_time = get_time() - start_time;
        let remaining_time = (1.0 / FPS) - elapsed_time as f32;

        if remaining_time > 0.0 {
            wait_seconds(remaining_time).await;
        }
    }
}

enum MenuState {
    Start,
    CloudDispersing,
    GameOver,
    IntroStart,
    IntroEmployeeEnter,
    IntroManagerWalk,
    IntroDoor,
    IntroManagerLeave,
    GameStart,
}
struct Menu {
    cloud1_pos: Vec2,
    cloud2_pos: Vec2,
    cloud1_start_pos: Vec2,
    cloud2_start_pos: Vec2,
    cloud1_end_pos: Vec2,
    cloud2_end_pos: Vec2,
    pub state: MenuState,
    spawning: bool,
    game_started: bool,
    tick_count: u64,
    crunch_mode: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            cloud1_pos: vec2(0., 0.),
            cloud2_pos: vec2(0., 0.),
            cloud1_start_pos: vec2(0., 0.),
            cloud2_start_pos: vec2(0., 0.),
            cloud1_end_pos: vec2(screen_width() * 8., 0.),
            cloud2_end_pos: vec2(-screen_width() * 8., 0.),
            state: MenuState::Start,
            spawning: false,
            game_started: false,
            tick_count: 0,
            crunch_mode: false,
        }
    }

    pub fn tick(&mut self, game: &mut Game) {
        self.tick_count += 1;

        match self.state {
            MenuState::Start => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let rect = Rect::new(0., 0., screen_width(), screen_height());
                    let main_pos =
                        Drawing::convert_screen_main(vec2(mouse_position().0, mouse_position().1));

                    if rect.contains(main_pos) {
                        self.state = MenuState::CloudDispersing;
                    }
                }
            }

            MenuState::CloudDispersing => {
                if self.cloud1_pos.x <= self.cloud1_end_pos.x {
                    self.cloud1_pos.x += 100.;
                }

                if self.cloud2_pos.x >= self.cloud2_end_pos.x {
                    self.cloud2_pos.x -= 100.;
                }

                if self.cloud1_pos.x >= self.cloud1_end_pos.x
                    && self.cloud2_pos.x <= self.cloud2_end_pos.x
                {
                    self.state = MenuState::IntroStart;
                }

                // Spawn employees
                if !self.spawning {
                    self.spawning = true;

                    let n = 15;
                    let n_employees = 3;

                    (0..n_employees).for_each(|_| {
                        game.office.add_employee_intro();
                        (0..n).for_each(|_| {
                            game.office.tick();
                        });
                    });
                }

                (0..4).for_each(|_| {
                    game.office.tick();
                });
            }
            MenuState::IntroStart => {
                println!("start");

                //

                self.state = MenuState::IntroEmployeeEnter;
            }
            MenuState::IntroEmployeeEnter => {
                (0..4).for_each(|_| {
                    game.office.tick();
                });

                if self.tick_count > 60 * 7 {
                    self.state = MenuState::IntroManagerWalk;
                }
            }
            MenuState::IntroManagerWalk => {
                (0..4).for_each(|_| {
                    game.office.tick();
                });

                if self.tick_count > 60 * 10 {
                    self.state = MenuState::IntroDoor;
                }
            }
            MenuState::IntroDoor => {
                (0..4).for_each(|_| {
                    game.office.tick();
                });

                if self.tick_count > 60 * 12 {
                    game.office.update_door();
                    self.state = MenuState::IntroManagerLeave;
                }
            }
            MenuState::IntroManagerLeave => {
                (0..4).for_each(|_| {
                    game.office.tick();
                });

                if self.tick_count > 60 * 15 {
                    self.state = MenuState::GameStart;
                }
            }
            MenuState::GameStart => {
                game.office.iter_employees_mut().for_each(|mut e| {
                    e.is_state_freezed = false;
                });

                self.game_started = true;
            }
            MenuState::GameOver => {
                if self.cloud1_pos.x >= self.cloud1_start_pos.x {
                    self.cloud1_pos.x -= 100.;
                }

                if self.cloud2_pos.x <= self.cloud2_start_pos.x {
                    self.cloud2_pos.x += 100.;
                }

                if self.cloud1_pos.x <= self.cloud1_start_pos.x
                    && self.cloud2_pos.x >= self.cloud2_start_pos.x
                {
                    self.state = MenuState::Start;
                }
            }
        }
    }

    pub fn draw(&mut self, game: &mut Game) {
        self.draw_clouds(game);
        match self.state {
            MenuState::GameOver => {
                // TODO Draw game over
            }
            _ => {
                if self.crunch_mode {
                    self.draw_logo2();
                } else {
                    self.draw_logo1();
                }
            }
        }
    }

    pub fn draw_logo1(&mut self) {
        draw_texture_ex(
            &assets::LOGO1_TEXTURE,
            screen_width() / 2. - 200.,
            screen_height() / 2. - 80.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(400., 160.)),
                ..Default::default()
            },
        );
    }

    pub fn draw_logo2(&mut self) {
        draw_texture_ex(
            &assets::LOGO2_TEXTURE,
            screen_width() / 2. - 200.,
            screen_height() / 2. - 80.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(400., 160.)),
                ..Default::default()
            },
        );
    }

    pub fn draw_clouds(&mut self, game: &mut Game) {
        let c = game.drawing.clone();
        let mut d = c.borrow_mut();

        d.draw_menu(game);

        draw_texture_ex(
            &assets::CLOUD_TEXTURE,
            -(screen_width() * (0.5)) + self.cloud1_pos.x,
            -(screen_height() * (1.8)) + self.cloud1_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width() * 2., screen_height() * 3.)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &assets::CLOUD2_TEXTURE,
            -(screen_width() * (0.7)) + self.cloud2_pos.x,
            -(screen_height() * (1.)) + self.cloud2_pos.y,
            WHITE,
            //
            DrawTextureParams {
                dest_size: Some(vec2(screen_width() * 2., screen_height() * 3.)),
                ..Default::default()
            },
        );
    }
}
