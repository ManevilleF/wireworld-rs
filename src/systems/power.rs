use crate::components::PowerGenerator;
use bevy::prelude::*;
use bevy_life::WireWorldCellState;

pub fn handle_power_generation(
    mut generators: Query<(&mut PowerGenerator, &mut WireWorldCellState)>,
    time: Res<Time>,
) {
    let time = time.time_since_startup().as_secs_f64();
    for (mut generator, mut state) in generators.iter_mut() {
        if generator.try_to_iterate(time) {
            *state = WireWorldCellState::ElectronHead
        }
    }
}
