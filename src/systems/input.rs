use crate::common::*;
use crate::resources::MapEntity;
use crate::systems::setup::spawn_map;
use bevy::prelude::*;
use bevy_life::{CellMap, MooreCell2d, WireWorldCellState};

pub fn handle_mouse_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&MooreCell2d, &mut WireWorldCellState)>,
    windows: Res<Windows>,
    map: Res<MapEntity>,
) {
    let sprite_size = 20;
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    let window = windows.get_primary().unwrap();
    let mouse_position = match window.cursor_position() {
        None => return,
        Some(p) => mouse_coords(window, p),
    };
    let position = mouse_coords_to_cell(mouse_position, sprite_size);
    let mut found_cell_state = None;
    for (cell, state) in query.iter_mut() {
        if cell.coords == position {
            found_cell_state = Some(state);
            break;
        }
    }
    if let Some(mut state) = found_cell_state {
        println!(
            "Cell at {:?} ({:?}) becomes an electron head",
            position, mouse_position
        );
        *state = WireWorldCellState::ElectronHead;
    } else {
        println!(
            "Spawning new conductor at {:?} ({:?}) ",
            position, mouse_position
        );
        commands.entity(map.0).with_children(|builder| {
            builder
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::splat((sprite_size - 1) as f32)),
                    transform: Transform::from_xyz(
                        (sprite_size * position.x) as f32,
                        (sprite_size * position.y) as f32,
                        0.,
                    ),
                    ..Default::default()
                })
                .insert(MooreCell2d::new(position))
                .insert(WireWorldCellState::Conductor);
        });
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
