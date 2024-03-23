use std::{
    cell::{Ref, RefCell},
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

pub struct Office {
    available_computers: Vec<Rc<RefCell<Computer>>>,
    employees: Vec<Rc<RefCell<Employee>>>,
    selected_employee: Option<Rc<RefCell<Employee>>>,
    money: u64,
}

impl Office {
    pub fn new() -> Self {
        let spot_x = [450., 600., 720., 860.];
        let spot_y = [175., 265., 435., 530.];

        let available_computers = spot_x
            .iter()
            .flat_map(|&x| {
                spot_y.iter().map(move |&y| {
                    Rc::new(RefCell::new(Computer::new(
                        Vec2::new(0., 0.),
                        Vec2::new(x, y),
                    )))
                })
            })
            .collect::<Vec<Rc<RefCell<Computer>>>>();

        Self {
            available_computers,
            employees: Vec::new(),
            selected_employee: None,
            money: 100,
        }
    }

    pub fn add_employee(&mut self) {
        let spot_index = rand::gen_range(0, self.available_computers.len());

        let employee_spot = self.available_computers.remove(spot_index);

        self.employees
            .push(Rc::new(RefCell::new(Employee::new(employee_spot))));
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

    pub fn iter_employees(&self) -> impl Iterator<Item = Ref<'_, Employee>> {
        self.employees.iter().map(|e| e.borrow())
    }

    pub fn iter_computers(&self) -> impl Iterator<Item = Rc<RefCell<Computer>>> + '_ {
        self.available_computers
            .iter()
            .cloned()
            .chain(self.employees.iter().map(|e| e.borrow().computer.clone()))
            .map(|c| c)
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

            if let EmployeeState::Remove = e_borrow.state {
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

    pub fn get_money(&self) -> u64 {
        self.money
    }
}

const BASE_DECAY_RATE: f32 = 0.0001;
const REPLENISH_RATE: f32 = BASE_DECAY_RATE * 10.;

pub const EMPLOYEE_RADIUS: f32 = 50.;
const EMPLOYEE_SPEED: f32 = 1.;

pub struct Computer {
    position: Vec2,
    brocken: bool,
    spot: Vec2,
}

impl Computer {
    pub fn new(position: Vec2, spot: Vec2) -> Self {
        Self {
            position,
            brocken: false,
            spot,
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
    Remove,
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
    position: Vec2,
    computer: Rc<RefCell<Computer>>,
    rotation: f32,
    state: EmployeeState,
    pub action: EmployeeAction,
}

impl Employee {
    pub fn new(computer: Rc<RefCell<Computer>>) -> Self {
        let name = NAMES[gen_range(0, NAMES.len())].to_owned();

        Self {
            name,
            satisfaction: 0.5,
            hope: 0.5,
            energy: 0.5,
            satiety: 0.5,
            position: Vec2::new(300., 350.),
            computer,
            rotation: 0.,
            state: EmployeeState::Alive,
            action: EmployeeAction::None,
        }
    }

    #[must_use]
    pub fn tick(&mut self) -> u64 {
        self.satisfaction -= BASE_DECAY_RATE;
        self.hope -= BASE_DECAY_RATE;
        self.energy -= BASE_DECAY_RATE;
        self.satiety -= BASE_DECAY_RATE;

        match self.action {
            EmployeeAction::None => (),
            EmployeeAction::Break => self.satisfaction += REPLENISH_RATE,
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
