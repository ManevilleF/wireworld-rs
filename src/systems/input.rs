use crate::common::*;
use crate::components::MouseTarget;
use crate::resources::{BoardMaterials, CameraTranslation, MapEntity, SpawnSelection};
use crate::CELL_SIZE;
use bevy::input::mouse::MouseWheel;
use bevy::log;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_life::{CellMap, MooreCell2d};

#[derive(Debug, Copy, Clone)]
enum InputMode {
    Creation,
    Deletion,
}

pub fn handle_zoom(
    mut camera_translation: ResMut<CameraTranslation>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut input_evr: EventReader<MouseWheel>,
) {
    for input in input_evr.iter() {
        let mut transform = camera_query.single_mut();
        transform.translation.x += input.x;
        transform.translation.y += input.y;
        camera_translation.0 = Vec2::new(transform.translation.x, transform.translation.y);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    cell_query: Query<&MooreCell2d>,
    mut selector_query: Query<&mut Transform, With<MouseTarget>>,
    windows: Res<Windows>,
    map: Res<MapEntity>,
    spawn_selection: Res<SpawnSelection>,
    mut cell_map: ResMut<CellMap<MooreCell2d>>,
    materials: Res<BoardMaterials>,
    camera_translation: Res<CameraTranslation>,
) {
    let input_mode = if mouse_input.pressed(MouseButton::Left) {
        Some(InputMode::Creation)
    } else if mouse_input.pressed(MouseButton::Right) {
        Some(InputMode::Deletion)
    } else {
        None
    };
    let window = windows.get_primary().unwrap();
    let mouse_position = match window.cursor_position() {
        None => return,
        Some(p) => mouse_coords(window, p) + camera_translation.0,
    };
    let position = mouse_coords_to_cell(mouse_position, CELL_SIZE as i32);

    match selector_query.get_single_mut().ok() {
        Some(mut t) => *t = MouseTarget::transform_value(position),
        None => {
            commands.spawn_bundle(MouseTarget::bundle(&materials, position));
        }
    };

    // Apply creation and deletion
    if let Some(mode) = input_mode {
        match mode {
            InputMode::Creation => {
                if cell_map.get_cell(&position).is_some() {
                    return;
                }
                log::info!(
                    "Spawning new conductor at {:?} ({:?}) ",
                    position,
                    mouse_position
                );
                commands.entity(map.0).with_children(|builder| {
                    let entity = spawn_selection.spawn_bundle(builder, position, CELL_SIZE as f32);
                    cell_map.insert_cell(position, entity);
                });
            }
            InputMode::Deletion => {
                for cell in cell_query.iter() {
                    if cell.coords == position {
                        log::info!("Deleting cell at {:?}", cell.coords);
                        if let Some(entity) = cell_map.remove_cell(&cell.coords) {
                            commands.entity(entity).despawn_recursive();
                        } else {
                            log::warn!("Tried to remove non existent cell at {:?}", cell.coords)
                        }
                        break;
                    }
                }
            }
        }
    }
}

pub fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut camera_translation: ResMut<CameraTranslation>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if keys.just_released(KeyCode::Space) {
        let mut transform = camera_query.single_mut();
        transform.translation.x = 0.;
        transform.translation.y = 0.;
        camera_translation.0 = Vec2::ZERO;
    }
}
