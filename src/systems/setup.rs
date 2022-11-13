use crate::resources::{BoardMaterials, CameraTranslation, ConductorBundle, MapEntity};
use crate::CELL_SIZE;
use bevy::prelude::*;
use bevy_life::{MooreCell2d, WireWorldCellState};

pub fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(CameraTranslation(Vec2::ZERO))
}

pub fn setup_map(mut commands: Commands) {
    commands.insert_resource(BoardMaterials::new());
    // map
    spawn_map(&mut commands);
}

pub fn spawn_map(commands: &mut Commands) {
    let map_size = 5;

    let entity = commands
        .spawn(SpatialBundle::default())
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
                    builder.spawn(ConductorBundle {
                        sprite_bundle: SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(CELL_SIZE - 1.)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                CELL_SIZE * x as f32,
                                CELL_SIZE * y as f32,
                                0.,
                            ),
                            ..Default::default()
                        },
                        cell: MooreCell2d::new(IVec2::new(x, y)),
                        state,
                    });
                }
            }
        })
        .id();
    commands.insert_resource(MapEntity(entity));
    println!("map generated");
}
