use bevy::prelude::{Resource, Vec2};

#[derive(Debug, Clone, Resource)]
pub struct CameraTranslation(pub Vec2);
