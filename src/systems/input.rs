use crate::common::*;
use crate::components::{MouseTarget, PowerGenerator};
use crate::events::{ClearWorld, RemoveCell, ResetCamera, SpawnCell};
use crate::resources::BoardMaterials;
use crate::CELL_SIZE;
use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
enum CellAction {
    SpawnConductor,
    SpawnGenerator,
    Remove,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_input(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut selector_query: Query<&mut Transform, With<MouseTarget>>,
    windows: Query<&Window>,
    materials: Res<BoardMaterials>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut spawn_cell_evw: EventWriter<SpawnCell>,
    mut remove_cell_evw: EventWriter<RemoveCell>,
) {
    let input_mode = if mouse_input.pressed(MouseButton::Left) {
        Some(CellAction::SpawnConductor)
    } else if mouse_input.pressed(MouseButton::Middle) {
        Some(CellAction::SpawnGenerator)
    } else if mouse_input.pressed(MouseButton::Right) {
        Some(CellAction::Remove)
    } else {
        None
    };
    let window = windows.single();
    let (camera, cam_transform) = camera.single();
    let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    else {
        return;
    };
    let position = mouse_coords_to_cell(pos, CELL_SIZE as i32);

    match selector_query.get_single_mut().ok() {
        Some(mut t) => *t = MouseTarget::transform_value(position),
        None => {
            commands.spawn(MouseTarget::bundle(&materials, position));
        }
    };

    // Apply creation and deletion
    if let Some(mode) = input_mode {
        match mode {
            CellAction::SpawnGenerator => {
                spawn_cell_evw.send(SpawnCell {
                    pos: position,
                    generator: Some(PowerGenerator::default()),
                });
            }
            CellAction::SpawnConductor => {
                spawn_cell_evw.send(SpawnCell {
                    pos: position,
                    generator: None,
                });
            }
            CellAction::Remove => {
                remove_cell_evw.send(RemoveCell { pos: position });
            }
        }
    }
}

pub fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut reset_cam_evw: EventWriter<ResetCamera>,
    mut clear_map_evw: EventWriter<ClearWorld>,
) {
    if keys.just_pressed(KeyCode::Space) {
        reset_cam_evw.send(ResetCamera);
    }
    if keys.just_pressed(KeyCode::Backspace) {
        clear_map_evw.send(ClearWorld);
    }
}
