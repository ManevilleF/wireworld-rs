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
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireWorld2dPlugin::new(WIRE_WORLD_TIME_STEP))
        .insert_resource(WindowDescriptor {
            title: "WireWorld".to_string(),
            width: 1000.,
            height: 1000.,
            ..Default::default()
        })
        .insert_resource(SpawnSelection::Conductor)
        .add_startup_system(systems::setup::setup_camera.system())
        .add_startup_system(systems::setup::setup_map.system())
        .add_system(systems::input::handle_keyboard_input.system())
        .add_system(systems::input::handle_mouse_input.system())
        .add_system(systems::input::handle_zoom.system())
        .add_system(systems::power::handle_power_generation.system())
        .add_system(systems::coloring::color_states.system())
        .add_system(systems::ui::handle_buttons.system())
        .add_event::<ButtonAction>()
        .run();
}
