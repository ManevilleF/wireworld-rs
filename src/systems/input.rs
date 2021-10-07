use crate::common::*;
use crate::resources::{MapEntity, SpawnSelection};
use crate::systems::setup::spawn_map;
use bevy::prelude::*;
use bevy_life::{CellMap, MooreCell2d};

#[derive(Debug, Copy, Clone)]
enum InputMode {
    Creation,
    Deletion,
}

pub fn handle_mouse_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    query: Query<(Entity, &MooreCell2d)>,
    windows: Res<Windows>,
    map: Res<MapEntity>,
    spawn_selection: Res<SpawnSelection>,
    mut cell_map: ResMut<CellMap<MooreCell2d>>,
) {
    let sprite_size = 20;
    let input_mode = if mouse_input.just_pressed(MouseButton::Left) {
        InputMode::Creation
    } else if mouse_input.just_pressed(MouseButton::Right) {
        InputMode::Deletion
    } else {
        return;
    };
    let window = windows.get_primary().unwrap();
    let mouse_position = match window.cursor_position() {
        None => return,
        Some(p) => mouse_coords(window, p),
    };
    let position = mouse_coords_to_cell(mouse_position, sprite_size);
    match input_mode {
        InputMode::Creation => {
            if cell_map.get_cell(&position).is_some() {
                return;
            }
            println!(
                "Spawning new conductor at {:?} ({:?}) ",
                position, mouse_position
            );
            commands.entity(map.0).with_children(|mut builder| {
                spawn_selection.spawn_bundle(&mut builder, position, sprite_size as f32);
            });
        }
        InputMode::Deletion => {
            for (entity, cell) in query.iter() {
                if cell.coords == position {
                    cell_map.remove_cell(&cell.coords);
                    commands.entity(entity).despawn_recursive();
                    break;
                }
            }
        }
    }
}

pub fn handle_reset(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    map: Res<MapEntity>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    mut cell_map: ResMut<CellMap<MooreCell2d>>,
) {
    if keys.just_released(KeyCode::Space) {
        commands.entity(map.0).despawn_recursive();
        commands.remove_resource::<MapEntity>();
        cell_map.clear();
        println!("regenerating map");
        spawn_map(&mut commands, &mut assets);
    }
}
