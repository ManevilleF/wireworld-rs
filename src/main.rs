pub mod common;
mod components;
mod events;
mod resources;
mod systems;

use std::time::Duration;

use crate::events::*;
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_life::WireWorld2dPlugin;

// TODO: allow env var
const WIRE_WORLD_TIME_STEP: f64 = 0.05;
pub const CELL_SIZE: f32 = 20.;

#[derive(Debug, Copy, Clone)]
pub enum AppState {
    Pause,
    Running,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "WireWorld".to_string(),
                resolution: (1000., 1000.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WireWorld2dPlugin::new()
                .with_time_step(WIRE_WORLD_TIME_STEP)
                .with_cell_map(),
        )
        .add_systems(Startup, (systems::camera::setup, systems::cells::setup))
        .add_systems(
            PreUpdate,
            (
                systems::input::handle_mouse_input,
                systems::input::handle_keyboard_input,
            )
                .after(InputSystem),
        )
        .add_systems(
            Update,
            (
                systems::camera::reset,
                systems::camera::zoom,
                systems::camera::movement,
                systems::coloring::color_states,
                systems::cells::spawn_cells,
                systems::cells::remove_cells,
                systems::cells::clear_map,
            ),
        )
        .add_systems(
            FixedUpdate,
            systems::power::handle_power_generation
                .run_if(on_timer(Duration::from_secs_f64(WIRE_WORLD_TIME_STEP))),
        )
        .add_event::<ClearWorld>()
        .add_event::<ResetCamera>()
        .add_event::<SpawnCell>()
        .add_event::<RemoveCell>()
        .run();
}
