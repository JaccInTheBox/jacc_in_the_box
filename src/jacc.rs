use rand::Rng;

pub struct Jacc {
    jacc_state: JaccState,
    clown_rng: u32,
    timer: u32,
    time_since_pop: u32
}

impl Jacc {
    pub fn new() -> Self {
        Jacc {
            jacc_state: JaccState::NotApplicable,
            clown_rng: get_rand_value(),
            timer: 0,
            time_since_pop: 0
        }
    }

    pub fn increment_timer(&mut self) {
        self.timer += 1;
    }

    pub fn increment_time_since_pop(&mut self) {
        self.time_since_pop += 1;
    }

    pub fn get_clown_rng(&self) -> u32 {
        self.clown_rng
    }

    pub fn get_jacc_state(&self) -> &JaccState {
        &self.jacc_state
    }

    pub fn get_timer(&self) -> u32 {
        self.timer
    }

    pub fn set_jacc_state_in_box(&mut self) {
        self.jacc_state = JaccState::InTheBox;
    }

    pub fn set_jacc_state_out_of_box(&mut self) {
        self.jacc_state = JaccState::OutOfBox;
    }
}

fn get_rand_value() -> u32 {
    rand::thread_rng().gen_range(1..15)
}

pub enum JaccState {
    NotApplicable,
    InTheBox,
    OutOfBox
}