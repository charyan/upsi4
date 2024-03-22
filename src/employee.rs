use std::{cell::RefCell, rc::Rc};

use macroquad::{math::Vec2, rand};

pub struct Office {
    available_spots: Vec<Vec2>,
    employees: Vec<Rc<RefCell<Employee>>>,
    selected_employee: Option<Rc<RefCell<Employee>>>,
}

impl Office {
    pub fn new() -> Self {
        let spot_x = [1., 3., 5., 7.];
        let spot_y = [1., 2., 4., 5.];

        let available_spots = spot_x
            .iter()
            .flat_map(|&x| spot_y.iter().map(move |&y| Vec2::new(x, y)))
            .collect::<Vec<Vec2>>();

        Self {
            available_spots,
            employees: Vec::new(),
            selected_employee: None,
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

    pub fn tick(&mut self) {
        for e in &self.employees {
            e.borrow_mut().tick();
        }
    }
}

const BASE_DECAY_RATE: f32 = 0.0001;
const EMPLOYEE_RADIUS: f32 = 1.;

pub struct Employee {
    satisfaction: f32,
    hope: f32,
    energy: f32,
    satiety: f32,
    position: Vec2,
    spot: Vec2,
    rotation: f32,
    alive: bool,
}

impl Employee {
    pub fn new(spot: Vec2) -> Self {
        Self {
            satisfaction: 0.5,
            hope: 0.5,
            energy: 0.5,
            satiety: 0.5,
            position: Vec2::new(0., 0.),
            spot,
            rotation: 0.,
            alive: true,
        }
    }

    pub fn tick(&mut self) {
        self.satisfaction -= BASE_DECAY_RATE;
        self.hope -= BASE_DECAY_RATE;
        self.energy -= BASE_DECAY_RATE;
        self.satiety -= BASE_DECAY_RATE;
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
}
