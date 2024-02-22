use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub struct PowerGenerator {
    pub ticks: u32,
    iteration_count: usize,
    tmp_ticks: u32,
}

impl PowerGenerator {
    pub fn new(ticks: u32) -> Self {
        Self {
            ticks,
            ..Default::default()
        }
    }

    pub fn tick(&mut self) -> bool {
        self.tmp_ticks = self.tmp_ticks.saturating_sub(1);
        if self.tmp_ticks == 0 {
            self.iteration_count += 1;
            self.tmp_ticks = self.ticks;
            true
        } else {
            false
        }
    }
}

impl Default for PowerGenerator {
    fn default() -> Self {
        Self {
            ticks: 10,
            iteration_count: 0,
            tmp_ticks: 0,
        }
    }
}
