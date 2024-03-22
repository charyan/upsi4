use macroquad::math::Vec2;

pub struct Office {
    available_spots: Vec<Vec2>,
    employees: Vec<Employee>,
}

impl Office {
    pub fn new() -> Self {
        let spot_x = vec![1., 3., 5., 7.];
        let spot_y = vec![1., 2., 4., 5.];

        let available_spots = spot_x
            .iter()
            .flat_map(|&x| spot_y.iter().map(move |&y| Vec2::new(x, y)))
            .collect::<Vec<Vec2>>();

        Self {
            available_spots,
            employees: Vec::new(),
        }
    }

    pub fn add_employee(&mut self) {}

    pub fn tick(&mut self) {
        for e in &mut self.employees {
            e.tick();
        }
    }
}

const BASE_DECAY_RATE: f32 = 0.0001;

pub struct Employee {
    satisfaction: f32,
    hope: f32,
    energy: f32,
    satiety: f32,
    position: Vec2,
    spot: Vec2,
    rotation: f32,
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
        }
    }

    pub fn tick(&mut self) {
        self.satisfaction -= BASE_DECAY_RATE;
        self.hope -= BASE_DECAY_RATE;
        self.energy -= BASE_DECAY_RATE;
        self.satiety -= BASE_DECAY_RATE;
    }
}
