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
            window: WindowDescriptor {
                title: "WireWorld".to_string(),
                width: 1000.,
                height: 1000.,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(WireWorld2dPlugin::new(WIRE_WORLD_TIME_STEP))
        .insert_resource(SpawnSelection::Conductor)
        .add_startup_system(systems::setup::setup_camera)
        .add_startup_system(systems::setup::setup_map)
        .add_system(systems::input::handle_keyboard_input)
        .add_system(systems::input::handle_mouse_input)
        .add_system(systems::input::handle_zoom)
        .add_system(systems::power::handle_power_generation)
        .add_system(systems::coloring::color_states)
        .add_system(systems::ui::handle_buttons)
        .add_event::<ButtonAction>()
        .run();
}
