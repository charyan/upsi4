use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig, BlendMode, Emitter, EmitterConfig};

use std::{
    cell::{Ref, RefCell, RefMut},
    f32::consts::PI,
    rc::Rc,
};

const NAMES: &[&str] = &[
    "Baptiste", "Yannis", "Valentin", "Cynthia", "Alain", "Valérie", "Yves", "Cédric", "Marcello",
    "Edsger", "Stephano", "Olivier", "Mathieu", "Valérie", "Roland", "Tom",
];

const BONUS_RH_COST: f32 = 1000.;

use macroquad::{
    math::Vec2,
    rand::{self, gen_range},
};

use crate::{assets, drawing::OFFICE_HEIGHT};

use crate::qte::QteEffect;

const SPOT_X: [f32; 4] = [450., 650., 750., 950.];
const MIDDLE_SPOT_X: [f32; 4] = [400., 700., 700., 1000.];
const SPOT_Y: [f32; 4] = [175., 265., 472., 551.];
pub const MIDDLE_LANE: f32 = 350.;
const WINDOW_X: f32 = 1090.;
const OPEN_WINDOW_X: f32 = 1000.;
const DOOR_X: f32 = 370.;
const SPEED_FALL: f32 = 10.;

#[derive(Clone, Copy, Debug)]
pub enum DoorState {
    /// The door is open
    Open,
    /// The door is closed
    Closed,
    /// The door is broken
    _Broken,
}

#[derive(Clone, Copy)]
pub enum GameState {
    Running,
    GameOver,
    MyLittleOfficeMenu,
    CrunchSimulatorMenu,
}

pub struct Office {
    available_computers: Vec<Rc<RefCell<Computer>>>,
    employees: Vec<Rc<RefCell<Employee>>>,
    selected_employee: Option<Rc<RefCell<Employee>>>,
    money: f32,
    door_state: DoorState,
    window_open: bool,
    game_state: GameState,
}

impl Office {
    pub fn new() -> Self {
        let mut new = Self {
            available_computers: Vec::new(),
            employees: Vec::new(),
            selected_employee: None,
            money: 100.,
            door_state: DoorState::Open,
            window_open: false,
            game_state: GameState::MyLittleOfficeMenu,
        };

        new.start();

        new
    }

    pub fn start(&mut self) {
        let computer_diff_with_spot_x = [60., -70., 65., -80.];

        self.available_computers = SPOT_X
            .iter()
            .enumerate()
            .flat_map(|(i, &x)| {
                SPOT_Y.iter().map(move |&y| {
                    Rc::new(RefCell::new(Computer::new(
                        Vec2::new(x + computer_diff_with_spot_x[i], y),
                        Vec2::new(x, y),
                        match i % 2 {
                            0 => PI / 2.,
                            _ => -PI / 2.,
                        },
                    )))
                })
            })
            .collect::<Vec<Rc<RefCell<Computer>>>>();

        self.employees.clear();
        self.selected_employee = None;
        self.money = 100.;
        self.door_state = DoorState::Open;
        self.window_open = false;
        self.game_state = GameState::Running;
    }

    pub fn add_employee(&mut self) {
        if self.available_computers.len() > 0 {
            let spot_index = rand::gen_range(0, self.available_computers.len());

            let employee_spot = self.available_computers.remove(spot_index);

            self.employees
                .push(Rc::new(RefCell::new(Employee::new(employee_spot))));
        }
    }

    pub fn get_selected_employee(&self) -> &Option<Rc<RefCell<Employee>>> {
        &self.selected_employee
    }

    pub fn click(&mut self, mouse_pos: Vec2) {
        self.selected_employee = None;

        for e in &self.employees {
            let borrowed_e = e.borrow();

            if borrowed_e.position.distance(mouse_pos) < EMPLOYEE_RADIUS {
                self.selected_employee = Some(e.clone());
            }
        }
    }

    pub fn kill_random_employee(&mut self) {
        let alive_employees = self
            .employees
            .iter()
            .filter(|&e| matches!(e.borrow().state, EmployeeState::Alive))
            .cloned()
            .collect::<Vec<Rc<RefCell<Employee>>>>();

        if alive_employees.len() > 0 {
            alive_employees[rand::gen_range(0, alive_employees.len())]
                .borrow_mut()
                .state = EmployeeState::Dead;
        }
    }

    pub fn update_door(&mut self) {
        if let DoorState::Open = self.door_state {
            self.door_state = DoorState::Closed;
        } else if let DoorState::Closed = self.door_state {
            self.door_state = DoorState::Open;
        }
    }

    pub fn update_window(&mut self) {
        self.window_open = !self.window_open
    }

    pub fn apply_qte_effect(&mut self, effect: &QteEffect) {
        self.money += effect.money_delta;

        for mut e in self.iter_employees_mut() {
            e.energy += effect.energy_delta;
            e.satisfaction += effect.satisfaction_delta;
            e.satiety += effect.satiety_delta;
            e.hope += effect.hope_delta;
        }

        if effect.employee_delta > 0 {
            for _ in 0..effect.employee_delta {
                self.add_employee()
            }
        }

        if effect.employee_delta < 0 {
            for _ in 0..(-effect.employee_delta) {
                self.kill_random_employee()
            }
        }
    }

    pub fn iter_employees(&self) -> impl Iterator<Item = Ref<'_, Employee>> {
        self.employees.iter().map(|e| e.borrow())
    }

    pub fn iter_employees_mut(&self) -> impl Iterator<Item = RefMut<'_, Employee>> {
        self.employees.iter().map(|e| e.borrow_mut())
    }

    pub fn iter_computers(&self) -> impl Iterator<Item = Rc<RefCell<Computer>>> + '_ {
        self.available_computers
            .iter()
            .cloned()
            .chain(self.employees.iter().map(|e| e.borrow().computer.clone()))
    }

    pub fn iter_computers_mut(&self) -> impl Iterator<Item = Rc<RefCell<Computer>>> + '_ {
        self.available_computers.iter().cloned().chain(
            self.employees
                .iter()
                .map(|e| e.borrow_mut().computer.clone()),
        )
    }

    pub fn employees_count(&self) -> usize {
        self.employees.len()
    }

    pub fn bonus_meth(&mut self) {
        if self.money >= BONUS_METH_COST {
            self.money -= BONUS_METH_COST;

            self.apply_qte_effect(&QteEffect::new(0.3, 0.3, 0.3, -0.3, 0., 0));
        }
    }

    pub fn tick(&mut self) {
        let mut removed_employees = Vec::new();

        let mut generated_money = 0.;

        if self.iter_employees().any(|x| x.get_pos().x > OPEN_WINDOW_X) {
            self.window_open = true;
        } else {
            self.window_open = false;
        }

        self.employees.retain(|e| {
            let mut e_borrow = e.borrow_mut();

            generated_money += e_borrow.tick(self.door_state);

            if let EmployeeState::Clean = e_borrow.state {
                removed_employees.push(e.clone());
                false
            } else {
                true
            }
        });

        self.money += generated_money;

        // Return spot to available spots
        for e in &removed_employees {
            self.available_computers.push(e.borrow().computer.clone());
        }

        // Deselect dead employee
        if let Some(selected) = &self.selected_employee {
            for removed in removed_employees {
                if Rc::ptr_eq(&removed, selected) {
                    self.selected_employee = None;
                    break;
                }
            }
        }

        let non_dead_employees_count = self
            .iter_employees()
            .filter(|e| !matches!(e.state, EmployeeState::Dead))
            .count();

        if non_dead_employees_count == 0 || self.money < 0. {
            self.game_state = GameState::GameOver;
        }
    }

    pub fn window_is_open(&self) -> bool {
        self.window_open
    }

    pub fn get_door_state(&self) -> &DoorState {
        &self.door_state
    }

    pub fn get_money(&self) -> f32 {
        self.money
    }

    pub fn bonus_rh(&mut self) {
        if self.money >= BONUS_RH_COST {
            self.money -= BONUS_RH_COST;
            self.add_employee();
        }
    }
}

const BASE_DECAY_RATE: f32 = 0.0001;
const REPLENISH_RATE: f32 = BASE_DECAY_RATE * 10.;

pub const EMPLOYEE_RADIUS: f32 = 50.;
const EMPLOYEE_SPEED: f32 = 1.;
const EMPLOYEE_RUNNING_SPEED: f32 = 3.;

const BONUS_METH_COST: f32 = 1000.;

fn feueur_particles() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.4,
        lifetime_randomness: 0.1,
        amount: 10,
        initial_direction_spread: 0.5,
        initial_velocity: 300.0,
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        size: 10.0,
        blend_mode: BlendMode::Additive,
        ..Default::default()
    }
}

pub struct Computer {
    pub position: Vec2,
    pub broken: bool,
    pub spot: Vec2,
    pub rotation: f32,
    pub emitter: Emitter,
}

impl Computer {
    pub fn new(position: Vec2, spot: Vec2, rotation: f32) -> Self {
        let emitter = Emitter::new(EmitterConfig {
            local_coords: false,
            texture: Some(assets::FEUER_TEXTURE.clone()),
            ..feueur_particles()
        });

        Self {
            position,
            broken: false,
            spot,
            rotation,
            emitter,
        }
    }
}

#[derive(Clone, Copy)]
pub enum EmployeeState {
    /// Normal employee state
    Alive,
    /// Dead state with option to reanimate or dispose of employee
    Dead,
    /// Employee thrown out of window
    Falling,
    /// Employee goes to the window
    Suicide,
    /// Internal state for when the entity can be removed from the world
    Clean,
}

#[derive(Clone, Copy)]
pub enum EmployeeAction {
    None,
    /// Satisfaction
    Break,
    /// Satiety
    Eat,
    /// Energy
    Sleep,
    /// Hope
    FamilyCall,
}

pub struct Employee {
    name: String,
    satisfaction: f32,
    hope: f32,
    energy: f32,
    satiety: f32,
    satisfaction_factor: f32,
    hope_factor: f32,
    energy_factor: f32,
    satiety_factor: f32,
    position: Vec2,
    computer: Rc<RefCell<Computer>>,
    rotation: f32,
    state: EmployeeState,
    movment_step: usize,
    pub action: EmployeeAction,
    pub z_emitter: Emitter,
    pub cry_emitter: Emitter,
    pub happy_emitter: Emitter,
}

fn sleep_particles() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 3.,
        lifetime_randomness: 0.1,
        amount: 3,
        initial_direction_spread: 5.,
        initial_velocity: 30.0,
        atlas: None,
        size: 15.0,
        blend_mode: BlendMode::Alpha,
        ..Default::default()
    }
}

fn cry_particles() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 1.,
        lifetime_randomness: 0.1,
        amount: 5,
        initial_direction_spread: 2.,
        initial_velocity: -50.0,
        atlas: None,
        size: 10.0,
        blend_mode: BlendMode::Alpha,
        ..Default::default()
    }
}

fn happy_particles() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 2.,
        lifetime_randomness: 0.1,
        amount: 3,
        initial_direction_spread: 2. * PI,
        initial_velocity: 20.0,
        atlas: None,
        size: 10.0,
        blend_mode: BlendMode::Alpha,
        ..Default::default()
    }
}

impl Employee {
    pub fn new(computer: Rc<RefCell<Computer>>) -> Self {
        let mut z_emitter = Emitter::new(EmitterConfig {
            local_coords: false,
            texture: Some(assets::Z_TEXTURE.clone()),
            ..sleep_particles()
        });

        let mut cry_emitter = Emitter::new(EmitterConfig {
            local_coords: false,
            texture: Some(assets::CRY_TEXTURE.clone()),
            ..cry_particles()
        });

        let mut happy_emitter = Emitter::new(EmitterConfig {
            local_coords: false,
            texture: Some(assets::HAPPY_TEXTURE.clone()),
            ..happy_particles()
        });

        let name = NAMES[gen_range(0, NAMES.len())].to_owned();

        Self {
            name,
            satisfaction: rand::gen_range(0.3, 0.7),
            hope: rand::gen_range(0.3, 0.7),
            energy: rand::gen_range(0.3, 0.7),
            satiety: rand::gen_range(0.3, 0.7),
            satisfaction_factor: rand::gen_range(0.7, 1.3),
            hope_factor: rand::gen_range(0.7, 1.3),
            energy_factor: rand::gen_range(0.7, 1.3),
            satiety_factor: rand::gen_range(0.7, 1.3),
            position: Vec2::new(300., MIDDLE_LANE),
            computer,
            rotation: 0.,
            state: EmployeeState::Alive,
            movment_step: 0,
            action: EmployeeAction::None,
            z_emitter,
            cry_emitter,
            happy_emitter,
        }
    }

    #[must_use]
    pub fn tick(&mut self, door_state: DoorState) -> f32 {
        if let EmployeeState::Clean = self.state {
            return 0.;
        }
        self.satisfaction -= BASE_DECAY_RATE * self.satisfaction_factor;
        self.hope -= BASE_DECAY_RATE * self.hope_factor * 2.;
        self.energy -= BASE_DECAY_RATE * self.energy_factor;
        self.satiety -= BASE_DECAY_RATE * self.satiety_factor;

        match self.action {
            EmployeeAction::None => (),
            EmployeeAction::Break => {
                self.satisfaction += REPLENISH_RATE;
            }
            EmployeeAction::Eat => self.satiety += REPLENISH_RATE,
            EmployeeAction::Sleep => self.energy += REPLENISH_RATE,
            EmployeeAction::FamilyCall => self.hope += REPLENISH_RATE,
        }

        self.satisfaction = self.satisfaction.clamp(0., 1.);
        self.hope = self.hope.clamp(0., 1.);
        self.energy = self.energy.clamp(0., 1.);
        self.satiety = self.satiety.clamp(0., 1.);

        if self.energy == 0. {
            self.action = EmployeeAction::Sleep
        }

        if self.satiety == 0. {
            self.state = EmployeeState::Dead
        }

        if self.hope == 0. && self.movment_step == 3 {
            self.state = EmployeeState::Suicide;
            if self.movment_step == 3 {
                self.movment_step = 0;
            }
        }

        if self.satisfaction == 0. && self.movment_step == 3 {
            self.computer.borrow_mut().broken = true;
        }

        if (self.satisfaction == 0. || self.hope == 1.) && self.movment_step == 3 {
            self.movment_step = 4;
        }

        let spot = self.computer.borrow().spot;

        if let EmployeeState::Alive = self.state {
            let index_x = SPOT_X.iter().position(|x: &f32| *x == spot.x).unwrap();

            match self.movment_step {
                0 => {
                    if self.position.x > MIDDLE_SPOT_X[index_x] {
                        self.position.x = MIDDLE_SPOT_X[index_x];
                        self.movment_step += 1;
                    } else {
                        self.position.x += EMPLOYEE_SPEED;
                        self.rotation = 0.;
                    }
                }
                1 => {
                    if spot.y < MIDDLE_LANE {
                        if self.position.y > spot.y {
                            self.position.y -= EMPLOYEE_SPEED;
                            self.rotation = -PI / 2.;
                        } else {
                            self.position.y = spot.y;
                            self.movment_step += 1;
                        }
                    } else {
                        if self.position.y < spot.y {
                            self.position.y += EMPLOYEE_SPEED;
                            self.rotation = PI / 2.;
                        } else {
                            self.position.y = spot.y;
                            self.movment_step += 1;
                        }
                    }
                }
                2 => {
                    if index_x % 2 == 0 {
                        if self.position.x < spot.x {
                            self.position.x += EMPLOYEE_SPEED;
                            self.rotation = 0.;
                        } else {
                            self.position.x = spot.x;
                            self.movment_step += 1;
                        }
                    } else {
                        if self.position.x > spot.x {
                            self.position.x -= EMPLOYEE_SPEED;
                            self.rotation = PI;
                        } else {
                            self.position.x = spot.x;
                            self.movment_step += 1;
                        }
                    }
                }
                3 => self.position = spot,
                4 => {
                    if index_x % 2 == 0 {
                        if self.position.x > MIDDLE_SPOT_X[index_x] {
                            self.position.x -= EMPLOYEE_RUNNING_SPEED;
                            self.rotation = PI;
                        } else {
                            self.position.x = MIDDLE_SPOT_X[index_x];
                            self.movment_step += 1;
                        }
                    } else {
                        if self.position.x < MIDDLE_SPOT_X[index_x] {
                            self.position.x += EMPLOYEE_RUNNING_SPEED;
                            self.rotation = 0.;
                        } else {
                            self.position.x = MIDDLE_SPOT_X[index_x];
                            self.movment_step += 1;
                        }
                    }
                }
                5 => {
                    if spot.y < MIDDLE_LANE {
                        if self.position.y < MIDDLE_LANE {
                            self.position.y += EMPLOYEE_RUNNING_SPEED;
                            self.rotation = PI / 2.;
                        } else {
                            self.position.y = MIDDLE_LANE;
                            self.movment_step += 1;
                        }
                    } else {
                        if self.position.y > MIDDLE_LANE {
                            self.position.y -= EMPLOYEE_RUNNING_SPEED;
                            self.rotation = -PI / 2.;
                        } else {
                            self.position.y = MIDDLE_LANE;
                            self.movment_step += 1;
                        }
                    }
                }
                6 => {
                    if self.position.x > DOOR_X {
                        self.position.x -= EMPLOYEE_RUNNING_SPEED;
                        self.rotation = PI
                    } else if let DoorState::Open = door_state {
                        self.movment_step += 1;
                    } else if self.hope < 0.9 && self.satisfaction > 0.1 {
                        self.movment_step = 0
                    } else {
                        self.position.x = DOOR_X;
                    }
                }
                7 => {
                    if self.position.x > 250. {
                        self.position.x -= EMPLOYEE_RUNNING_SPEED;
                    } else {
                        self.position.x = 250.;
                        self.movment_step += 1;
                    }
                }
                8 => {
                    self.rotation = -PI / 2.;
                    self.position.y -= EMPLOYEE_RUNNING_SPEED;
                    if self.position.y < 0. {
                        self.clean();
                    }
                }
                _ => (),
            }
        } else if let EmployeeState::Suicide = self.state {
            let index_x = SPOT_X.iter().position(|x: &f32| *x == spot.x).unwrap();

            match self.movment_step {
                0 => {
                    if index_x % 2 == 0 {
                        if self.position.x > MIDDLE_SPOT_X[index_x] {
                            self.position.x -= EMPLOYEE_SPEED;
                            self.rotation = PI;
                        } else {
                            self.position.x = MIDDLE_SPOT_X[index_x];
                            self.movment_step += 1;
                        }
                    } else {
                        if self.position.x < MIDDLE_SPOT_X[index_x] {
                            self.position.x += EMPLOYEE_SPEED;
                            self.rotation = 0.;
                        } else {
                            self.position.x = MIDDLE_SPOT_X[index_x];
                            self.movment_step += 1;
                        }
                    }
                }
                1 => {
                    if spot.y < MIDDLE_LANE {
                        if self.position.y < MIDDLE_LANE {
                            self.position.y += EMPLOYEE_SPEED;
                            self.rotation = PI / 2.;
                        } else {
                            self.position.y = MIDDLE_LANE;
                            self.movment_step += 1;
                        }
                    } else {
                        if self.position.y > MIDDLE_LANE {
                            self.position.y -= EMPLOYEE_SPEED;
                            self.rotation = -PI / 2.;
                        } else {
                            self.position.y = MIDDLE_LANE;
                            self.movment_step += 1;
                        }
                    }
                }
                2 => {
                    if self.position.x < WINDOW_X {
                        self.position.x += EMPLOYEE_SPEED;
                        self.rotation = 0.
                    } else {
                        self.position.x = WINDOW_X;
                        self.state = EmployeeState::Falling;
                    }
                }
                _ => (),
            }
        } else if let EmployeeState::Falling = self.state {
            if self.position.y < OFFICE_HEIGHT as f32 + 50. {
                self.position += vec2(21., 84.) / SPEED_FALL;
            } else {
                self.clean();
            }
        }

        if let EmployeeState::Alive = self.state {
            if self.computer.borrow().broken || self.movment_step != 3 {
                0.
            } else {
                if self.satisfaction == 1. {
                    0.01
                } else {
                    0.1
                }
            }
        } else {
            0.
        }
    }

    pub fn clean(&mut self) {
        self.state = EmployeeState::Clean;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_satisfaction(&self) -> f32 {
        self.satisfaction
    }

    pub fn get_hope(&self) -> f32 {
        self.hope
    }

    pub fn get_energy(&self) -> f32 {
        self.energy
    }

    pub fn get_satiety(&self) -> f32 {
        self.satiety
    }

    pub fn get_pos(&self) -> Vec2 {
        self.position
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_state(&self) -> EmployeeState {
        self.state
    }

    pub fn is_happy(&self) -> bool {
        self.satisfaction == 1.
    }

    pub fn is_mad(&self) -> bool {
        self.satisfaction == 0.
    }
}
