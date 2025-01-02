use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::events::ResetCamera;

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}

pub fn movement(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    button: Res<ButtonInput<KeyCode>>,
    mut input_evr: EventReader<MouseMotion>,
) {
    let motion: Vec2 = input_evr.read().map(|e| e.delta).sum();
    if button.pressed(KeyCode::ControlLeft) {
        let mut transform = camera_query.single_mut();
        transform.translation.x -= motion.x;
        transform.translation.y += motion.y;
    }
}

pub fn zoom(
    mut camera_query: Query<&mut OrthographicProjection>,
    mut input_evr: EventReader<MouseWheel>,
) {
    let motion: f32 = input_evr.read().map(|e| e.y).sum();
    if motion != 0.0 {
        let mut proj = camera_query.single_mut();
        proj.scale -= motion;
        proj.scale = proj.scale.clamp(0.1, 3.0);
        println!("{}", proj.scale);
    }
}

pub fn reset(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut reset_evr: EventReader<ResetCamera>,
) {
    if reset_evr.is_empty() {
        return;
    }
    let mut transform = camera_query.single_mut();
    transform.translation.x = 0.;
    transform.translation.y = 0.;
    reset_evr.clear();
}
