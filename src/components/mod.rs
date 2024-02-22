pub use {generator::*, mouse_target::*};

mod generator;
mod mouse_target;

use bevy::prelude::*;
use bevy_life::{MooreCell2d, WireWorldCellState};

#[derive(Bundle)]
pub struct CellBundle {
    pub sprite_bundle: SpriteBundle,
    pub cell: MooreCell2d,
    pub state: WireWorldCellState,
}

impl CellBundle {
    pub fn new(coordinates: IVec2, sprite_size: f32) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(sprite_size - 1.)),
                    ..Default::default()
                },
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
