pub mod common;
mod components;
mod resources;
mod systems;

use crate::components::ButtonAction;
use crate::resources::SpawnSelection;
use bevy::prelude::*;
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
        .add_plugin(WireWorld2dPlugin::with_time_step(WIRE_WORLD_TIME_STEP))
        .insert_resource(SpawnSelection::Conductor)
        .add_startup_system(systems::setup::setup_camera)
        .add_startup_system(systems::setup::setup_map)
        .add_systems((systems::input::handle_keyboard_input,
        systems::input::handle_mouse_input,
        systems::input::handle_zoom,
        systems::power::handle_power_generation,
        systems::coloring::color_states,
        systems::ui::handle_buttons))
        .add_event::<ButtonAction>()
        .run();
}
