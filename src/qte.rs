#[derive(Clone)]
pub struct QteEffect {
    pub satisfaction_delta: f32,
    pub energy_delta: f32,
    pub satiety_delta: f32,
    pub hope_delta: f32,
    pub money_delta: i64,
    pub employee_delta: i64,
}

impl QteEffect {
    pub fn new(
        satisfaction_delta: f32,
        energy_delta: f32,
        satiety_delta: f32,
        hope_delta: f32,
        money_delta: i64,
        employee_delta: i64,
    ) -> Self {
        Self {
            satisfaction_delta,
            energy_delta,
            satiety_delta,
            hope_delta,
            money_delta,
            employee_delta,
        }
    }
}

#[derive(Clone)]
pub struct QTE {
    text: String,
    effect_choice_1: QteEffect,
    effect_choice_2: QteEffect,
    choice1: String,
    choice2: String,
    explication1: String,
    explication2: String,
    time: f32,
    answer: Option<QteEffect>,
}

impl QTE {
    pub fn new(
        text: String,
        effect_choice_1: QteEffect,
        effect_choice_2: QteEffect,
        choice1: String,
        choice2: String,
        explication1: String,
        explication2: String,
        time: f32,
    ) -> Self {
        Self {
            text,
            effect_choice_1,
            effect_choice_2,
            choice1,
            choice2,
            explication1,
            explication2,
            time,
            answer: None,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_choice1(&self) -> &str {
        &self.choice1
    }

    pub fn get_choice2(&self) -> &str {
        &self.choice2
    }

    pub fn get_explication1(&self) -> &str {
        &self.explication1
    }

    pub fn get_explication2(&self) -> &str {
        &self.explication2
    }

    pub fn get_effect_1(&self) -> &QteEffect {
        &self.effect_choice_1
    }

    pub fn get_effect_2(&self) -> &QteEffect {
        &self.effect_choice_2
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }
}
