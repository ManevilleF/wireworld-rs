use crate::resources::BoardMaterials;
use bevy::prelude::*;
use bevy_life::WireWorldCellState;

pub fn color_states(
    mut query: Query<(&mut Sprite, &WireWorldCellState), Changed<WireWorldCellState>>,
    materials: Res<BoardMaterials>,
) {
    for (mut sprite, state) in query.iter_mut() {
        let new_color = match state {
            WireWorldCellState::Conductor => materials.conductor_material,
            WireWorldCellState::ElectronHead => materials.electron_head_material,
            WireWorldCellState::ElectronTail => materials.electron_tail_material,
        };
        sprite.color = new_color;
    }
}
