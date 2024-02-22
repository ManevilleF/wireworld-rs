use bevy::prelude::*;

use crate::components::PowerGenerator;

#[derive(Event, Debug, Clone, Copy)]
pub struct ClearWorld;

#[derive(Event, Debug, Clone, Copy)]
pub struct ResetCamera;

#[derive(Event, Debug, Clone)]
pub struct SpawnCell {
    pub pos: IVec2,
    pub generator: Option<PowerGenerator>,
}

#[derive(Event, Debug, Clone)]
pub struct RemoveCell {
    pub pos: IVec2,
}
