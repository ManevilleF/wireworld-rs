use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub enum ButtonAction {
    Clear,
    SpawnConductor,
    SpawnGenerator,
}
