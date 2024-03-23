use macroquad::prelude::*;
use macroquad_particles::{self as particles, BlendMode, Emitter, EmitterConfig};

use std::{
    cell::{Ref, RefCell, RefMut},
    f32::consts::PI,
    rc::Rc,
};

const NAMES: &[&str] = &[
    "Baptiste", "Yannis", "Valentin", "Cynthia", "Alain", "Valérie", "Yves", "Cédric", "Marcello",
    "Edsger", "Stephano", "Olivier", "Mathieu", "Valérie", "Roland", "Tom",
];

use macroquad::{
    math::Vec2,
    rand::{self, gen_range},
};

use crate::assets;

use crate::qte::QteEffect;

pub enum DoorState {
    /// The door is open
    Open,
    /// The door is closed
    Closed,
    /// The door is broken
    _Broken,
}

pub struct Office {
    available_computers: Vec<Rc<RefCell<Computer>>>,
    employees: Vec<Rc<RefCell<Employee>>>,
    selected_employee: Option<Rc<RefCell<Employee>>>,
    money: i64,
    door_state: DoorState,
    window_open: bool,
}

impl Office {
    pub fn new() -> Self {
        let spot_x = [450., 647., 743., 951.];
        let spot_y = [175., 265., 472., 551.];

        let computer_diff_with_spot_x = [60., -70., 65., -80.];

        let available_computers = spot_x
            .iter()
            .enumerate()
            .flat_map(|(i, &x)| {
                spot_y.iter().map(move |&y| {
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

        Self {
            available_computers,
            employees: Vec::new(),
            selected_employee: None,
            money: 100,
            door_state: DoorState::Open,
            window_open: false,
        }
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

    pub fn employees_count(&self) -> usize {
        self.employees.len()
    }

    pub fn tick(&mut self) {
        let mut removed_employees = Vec::new();

        let mut generated_money = 0;

        self.employees.retain(|e| {
            let mut e_borrow = e.borrow_mut();

            generated_money += e_borrow.tick();

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
    }

    pub fn window_is_open(&self) -> bool {
        self.window_open
    }

    pub fn get_door_state(&self) -> &DoorState {
        &self.door_state
    }

    pub fn get_money(&self) -> i64 {
        self.money
    }
}

const BASE_DECAY_RATE: f32 = 0.0001;
const REPLENISH_RATE: f32 = BASE_DECAY_RATE * 10.;

pub const EMPLOYEE_RADIUS: f32 = 50.;
const EMPLOYEE_SPEED: f32 = 1.;

pub struct Computer {
    pub position: Vec2,
    pub broken: bool,
    pub spot: Vec2,
    pub rotation: f32,
}

impl Computer {
    pub fn new(position: Vec2, spot: Vec2, rotation: f32) -> Self {
        Self {
            position,
            broken: false,
            spot,
            rotation,
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
    pub action: EmployeeAction,
    pub emitter: Emitter,
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

impl Employee {
    pub fn new(computer: Rc<RefCell<Computer>>) -> Self {
        let mut emitter = Emitter::new(EmitterConfig {
            local_coords: false,
            texture: Some(assets::Z_TEXTURE.clone()),
            ..sleep_particles()
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
            position: Vec2::new(300., 350.),
            computer,
            rotation: 0.,
            state: EmployeeState::Alive,
            action: EmployeeAction::None,
            emitter,
        }
    }

    #[must_use]
    pub fn tick(&mut self) -> i64 {
        if let EmployeeState::Clean = self.state {
            return 0;
        }

        self.satisfaction -= BASE_DECAY_RATE * self.satisfaction_factor;
        self.hope -= BASE_DECAY_RATE * self.hope_factor;
        self.energy -= BASE_DECAY_RATE * self.energy_factor;
        self.satiety -= BASE_DECAY_RATE * self.satiety_factor;

        match self.action {
            EmployeeAction::None => (),
            EmployeeAction::Break => {
                self.satisfaction += REPLENISH_RATE;
                self.computer.borrow_mut().broken = true;
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

        let spot = self.computer.borrow().spot;

        match self.position.x.total_cmp(&spot.x) {
            std::cmp::Ordering::Less => {
                if (spot.x - self.position.x) <= EMPLOYEE_SPEED {
                    self.position.x = spot.x;
                } else {
                    self.position.x += EMPLOYEE_SPEED;
                    self.rotation = 0.;
                }
            }
            std::cmp::Ordering::Equal => match self.position.y.total_cmp(&spot.y) {
                std::cmp::Ordering::Less => {
                    if (spot.y - self.position.y) <= EMPLOYEE_SPEED {
                        self.position.y = spot.y;
                    } else {
                        self.position.y += EMPLOYEE_SPEED;
                        self.rotation = PI / 2.;
                    }
                }
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => {
                    if (self.position.y - spot.y) <= EMPLOYEE_SPEED {
                        self.position.y = spot.y;
                    } else {
                        self.position.y -= EMPLOYEE_SPEED;
                        self.rotation = 3. * PI / 2.;
                    }
                }
            },
            std::cmp::Ordering::Greater => {
                if (self.position.x - spot.x) <= EMPLOYEE_SPEED {
                    self.position.x = spot.x;
                } else {
                    self.position.x -= EMPLOYEE_SPEED;
                    self.rotation = PI;
                }
            }
        }

        // TODO ! return generated amount of money
        if let EmployeeState::Alive = self.state {
            1
        } else {
            0
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
}
