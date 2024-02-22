use crate::components::PowerGenerator;
use bevy::prelude::*;
use bevy_life::WireWorldCellState;

pub fn handle_power_generation(
    mut generators: Query<(&mut PowerGenerator, &mut WireWorldCellState)>,
) {
    for (mut generator, mut state) in generators.iter_mut() {
        if generator.tick() {
            *state = WireWorldCellState::ElectronHead
        }
    }
}
