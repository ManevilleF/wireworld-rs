#[derive(Debug, Clone)]
pub struct PowerGenerator {
    pub delta_time: f64,
    iteration_count: usize,
    last_generation_time: f64,
}

impl PowerGenerator {
    pub fn new(frequency: f64) -> Self {
        Self {
            delta_time: 1.0 / frequency,
            ..Default::default()
        }
    }

    pub fn try_to_iterate(&mut self, time: f64) -> bool {
        if time - self.last_generation_time >= self.delta_time {
            self.new_iteration(time);
            true
        } else {
            false
        }
    }

    pub fn new_iteration(&mut self, time: f64) {
        self.iteration_count += 1;
        self.last_generation_time = time;
    }
}

impl Default for PowerGenerator {
    fn default() -> Self {
        Self {
            delta_time: 1.0,
            iteration_count: 0,
            last_generation_time: 0.0,
        }
    }
}
