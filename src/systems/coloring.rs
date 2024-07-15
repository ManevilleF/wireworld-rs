use crate::{components::PowerGenerator, resources::BoardMaterials};
use bevy::prelude::*;
use bevy_life::WireWorldCellState;

pub fn color_states(
    mut query: Query<
        (&mut Sprite, &WireWorldCellState, Has<PowerGenerator>),
        Changed<WireWorldCellState>,
    >,
    materials: Res<BoardMaterials>,
) {
    for (mut sprite, state, power) in query.iter_mut() {
        let new_color = match (power, state) {
            (false, WireWorldCellState::Conductor) => materials.conductor_material,
            (false, WireWorldCellState::ElectronHead) => materials.electron_head_material,
            (false, WireWorldCellState::ElectronTail) => materials.electron_tail_material,
            (true, _) => materials.generator_material,
        };
        sprite.color = new_color;
    }
}
