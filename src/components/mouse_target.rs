use crate::resources::BoardMaterials;
use crate::CELL_SIZE;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct MouseTarget;

#[derive(Bundle)]
pub struct MouseTargetBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    mouse_target: MouseTarget,
}

impl MouseTarget {
    pub fn bundle(materials: &BoardMaterials, position: IVec2) -> MouseTargetBundle {
        MouseTargetBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite::new(Vec2::splat(CELL_SIZE - 1.)),
                material: materials.selector_material.clone(),
                transform: Self::transform_value(position),
                ..Default::default()
            },
            mouse_target: Self,
        }
    }

    pub fn transform_value(position: IVec2) -> Transform {
        Transform::from_xyz(
            position.x as f32 * CELL_SIZE,
            position.y as f32 * CELL_SIZE,
            0.,
        )
    }
}
