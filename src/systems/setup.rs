use crate::resources::MapEntity;
use bevy::prelude::*;
use bevy_life::{MooreCell2d, WireWorldCellState};

pub fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn setup_map(mut commands: Commands, mut assets: ResMut<Assets<ColorMaterial>>) {
    // map
    spawn_map(&mut commands, &mut assets);
}

pub fn spawn_map(commands: &mut Commands, assets: &mut Assets<ColorMaterial>) {
    let map_size = 5;
    let sprite_size = 20.;
    let material = assets.add(Color::rgba(0., 0., 0., 0.).into());

    let entity = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|builder| {
            for y in -map_size..=map_size {
                for x in -map_size..=map_size {
                    if (x > -map_size && x < map_size && y > -map_size && y < map_size)
                        || ((x == -map_size || x == map_size) && (y == -map_size || y == map_size))
                    {
                        continue;
                    }
                    let state = if x == 0 && y == -map_size {
                        WireWorldCellState::ElectronTail
                    } else if x == 1 && y == -map_size {
                        WireWorldCellState::ElectronHead
                    } else {
                        WireWorldCellState::Conductor
                    };
                    builder
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::splat(sprite_size - 1.)),
                            transform: Transform::from_xyz(
                                sprite_size * x as f32,
                                sprite_size * y as f32,
                                0.,
                            ),
                            material: material.clone(),
                            ..Default::default()
                        })
                        .insert(MooreCell2d::new(IVec2::new(x, y)))
                        .insert(state);
                }
            }
        })
        .id();
    commands.insert_resource(MapEntity(entity));
    println!("map generated");
}
