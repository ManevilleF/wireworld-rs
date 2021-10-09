use crate::components::PowerGenerator;
use bevy::ecs::bundle::Bundle;
use bevy::math::IVec2;
use bevy::prelude::*;
use bevy_life::{MooreCell2d, WireWorldCellState};

pub enum SpawnSelection {
    Conductor,
    Generator { frequency: f64 },
}

impl SpawnSelection {
    pub fn spawn_bundle(
        &self,
        commands: &mut ChildBuilder,
        coordinates: IVec2,
        sprite_size: f32,
    ) -> Entity {
        match self {
            SpawnSelection::Conductor => commands
                .spawn_bundle(ConductorBundle::new(coordinates, sprite_size))
                .id(),
            SpawnSelection::Generator { frequency } => commands
                .spawn_bundle(GeneratorBundle {
                    conductor_bundle: ConductorBundle::new(coordinates, sprite_size),
                    generator: PowerGenerator::new(*frequency),
                })
                .id(),
        }
    }
}

#[derive(Bundle)]
pub struct ConductorBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub cell: MooreCell2d,
    pub state: WireWorldCellState,
}

#[derive(Bundle)]
pub struct GeneratorBundle {
    #[bundle]
    pub conductor_bundle: ConductorBundle,
    pub generator: PowerGenerator,
}

impl ConductorBundle {
    pub fn new(coordinates: IVec2, sprite_size: f32) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite::new(Vec2::splat(sprite_size - 1.)),
                transform: Transform::from_xyz(
                    coordinates.x as f32 * sprite_size,
                    coordinates.y as f32 * sprite_size,
                    0.,
                ),
                ..Default::default()
            },
            cell: MooreCell2d::new(coordinates),
            state: WireWorldCellState::Conductor,
        }
    }
}
