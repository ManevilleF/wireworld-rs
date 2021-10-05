pub mod common;
mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_life::WireWorld2dPlugin;

const WIREWORD_TIME_STEP: f64 = 0.05;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireWorld2dPlugin::new(WIREWORD_TIME_STEP))
        .insert_resource(WindowDescriptor {
            title: "WireWorld".to_string(),
            width: 1000.,
            height: 1000.,
            ..Default::default()
        })
        .add_startup_system(systems::setup::setup_camera.system())
        .add_startup_system(systems::setup::setup_map.system())
        .add_system(systems::input::handle_reset.system())
        .add_system(systems::input::handle_mouse_input.system())
        .run();
}
