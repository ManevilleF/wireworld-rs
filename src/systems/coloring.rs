use crate::resources::BoardMaterials;
use bevy::prelude::*;
use bevy_life::WireWorldCellState;

pub fn color_states(
    mut query: Query<
        (&mut Handle<ColorMaterial>, &WireWorldCellState),
        Changed<WireWorldCellState>,
    >,
    materials: Res<BoardMaterials>,
) {
    for (mut material, state) in query.iter_mut() {
        let new_handle = match state {
            WireWorldCellState::Conductor => materials.conductor_material.clone(),
            WireWorldCellState::ElectronHead => materials.electron_head_material.clone(),
            WireWorldCellState::ElectronTail => materials.electron_tail_material.clone(),
        };
        *material = new_handle;
    }
}
