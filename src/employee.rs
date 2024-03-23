use std::{
    cell::{Ref, RefCell},
    f32::consts::PI,
    rc::Rc,
};

use macroquad::{math::Vec2, rand};

pub struct Office {
    available_spots: Vec<Vec2>,
    employees: Vec<Rc<RefCell<Employee>>>,
    selected_employee: Option<Rc<RefCell<Employee>>>,
    money: u64,
}

impl Office {
    pub fn new() -> Self {
        let spot_x = [100., 300., 500., 700.];
        let spot_y = [100., 200., 400., 500.];

        let available_spots = spot_x
            .iter()
            .flat_map(|&x| spot_y.iter().map(move |&y| Vec2::new(x, y)))
            .collect::<Vec<Vec2>>();

        Self {
            available_spots,
            employees: Vec::new(),
            selected_employee: None,
            money: 100,
        }
    }

    pub fn add_employee(&mut self) {
        let spot_index = rand::gen_range(0, self.available_spots.len());

        let employee_spot = self.available_spots.remove(spot_index);

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

    pub fn employees_count(&self) -> usize {
        self.employees.len()
    }

    pub fn tick(&mut self) {
        let mut removed_employees = Vec::new();

        let mut generated_money = 0;

        self.employees.retain(|e| {
            let mut e_borrow = e.borrow_mut();

            generated_money += e_borrow.tick();

            if e_borrow.cleaned {
                removed_employees.push(e.clone());
                false
            } else {
                true
            }
        });

        self.money += generated_money;

        // Return spot to available spots
        for e in &removed_employees {
            self.available_spots.push(e.borrow().spot);
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
pub const EMPLOYEE_RADIUS: f32 = 150.;
const EMPLOYEE_SPEED: f32 = 1.;

pub struct Employee {
    satisfaction: f32,
    hope: f32,
    energy: f32,
    satiety: f32,
    position: Vec2,
    spot: Vec2,
    rotation: f32,
    alive: bool,
    cleaned: bool,
}

impl Employee {
    pub fn new(spot: Vec2) -> Self {
        Self {
            satisfaction: 0.5,
            hope: 0.5,
            energy: 0.5,
            satiety: 0.5,
            position: Vec2::new(10., 45.),
            spot,
            rotation: 0.,
            alive: true,
            cleaned: false,
        }
    }

    #[must_use]
    pub fn tick(&mut self) -> u64 {
        self.satisfaction -= BASE_DECAY_RATE;
        self.hope -= BASE_DECAY_RATE;
        self.energy -= BASE_DECAY_RATE;
        self.satiety -= BASE_DECAY_RATE;

        self.satisfaction = self.satisfaction.clamp(0., 1.);
        self.hope = self.hope.clamp(0., 1.);
        self.energy = self.energy.clamp(0., 1.);
        self.satiety = self.satiety.clamp(0., 1.);

        match self.position.x.total_cmp(&self.spot.x) {
            std::cmp::Ordering::Less => {
                if (self.spot.x - self.position.x) <= EMPLOYEE_SPEED {
                    self.position.x = self.spot.x;
                } else {
                    self.position.x += EMPLOYEE_SPEED;
                    self.rotation = 0.;
                }
            }
            std::cmp::Ordering::Equal => match self.position.y.total_cmp(&self.spot.y) {
                std::cmp::Ordering::Less => {
                    if (self.spot.y - self.position.y) <= EMPLOYEE_SPEED {
                        self.position.y = self.spot.y;
                    } else {
                        self.position.y += EMPLOYEE_SPEED;
                        self.rotation = PI / 2.;
                    }
                }
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => {
                    if (self.position.y - self.spot.y) <= EMPLOYEE_SPEED {
                        self.position.y = self.spot.y;
                    } else {
                        self.position.y -= EMPLOYEE_SPEED;
                        self.rotation = 3. * PI / 2.;
                    }
                }
            },
            std::cmp::Ordering::Greater => {
                if (self.position.x - self.spot.x) <= EMPLOYEE_SPEED {
                    self.position.x = self.spot.x;
                } else {
                    self.position.x -= EMPLOYEE_SPEED;
                    self.rotation = PI;
                }
            }
        }

        1 // TODO ! return generated amount of money
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
}
