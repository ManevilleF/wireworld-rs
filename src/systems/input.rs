use crate::common::*;
use crate::components::MouseTarget;
use crate::resources::{BoardMaterials, MapEntity, SpawnSelection};
use crate::systems::setup::spawn_map;
use crate::CELL_SIZE;
use bevy::log;
use bevy::prelude::*;
use bevy_life::{CellMap, MooreCell2d};

#[derive(Debug, Copy, Clone)]
enum InputMode {
    Creation,
    Deletion,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    cell_query: Query<(Entity, &MooreCell2d)>,
    mut selector_query: Query<&mut Transform, With<MouseTarget>>,
    windows: Res<Windows>,
    map: Res<MapEntity>,
    spawn_selection: Res<SpawnSelection>,
    mut cell_map: ResMut<CellMap<MooreCell2d>>,
    materials: Res<BoardMaterials>,
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
        Some(p) => mouse_coords(window, p),
    };
    let position = mouse_coords_to_cell(mouse_position, CELL_SIZE as i32);

    match selector_query.single_mut().ok() {
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
                commands.entity(map.0).with_children(|mut builder| {
                    let entity =
                        spawn_selection.spawn_bundle(&mut builder, position, CELL_SIZE as f32);
                    cell_map.insert_cell(position, entity);
                });
            }
            InputMode::Deletion => {
                for (entity, cell) in cell_query.iter() {
                    if cell.coords == position {
                        log::info!("Deleting cell at {:?}", cell.coords);
                        cell_map.remove_cell(&cell.coords);
                        commands.entity(entity).despawn_recursive();
                        break;
                    }
                }
            }
        }
    }
}

pub fn handle_reset(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    map: Res<MapEntity>,
    mut cell_map: ResMut<CellMap<MooreCell2d>>,
) {
    if keys.just_released(KeyCode::Space) {
        commands.entity(map.0).despawn_recursive();
        commands.remove_resource::<MapEntity>();
        cell_map.clear();
        log::info!("regenerating map");
        spawn_map(&mut commands);
    }
}
