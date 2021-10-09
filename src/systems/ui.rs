use crate::components::ButtonAction;
use crate::resources::{MapEntity, SpawnSelection};
use crate::systems::setup::spawn_map;
use bevy::log;
use bevy::prelude::*;
use bevy_life::{CellMap, MooreCell2d};

pub fn handle_buttons(
    mut commands: Commands,
    map: Res<MapEntity>,
    mut cell_map: ResMut<CellMap<MooreCell2d>>,
    mut button_evr: EventReader<ButtonAction>,
) {
    for button_event in button_evr.iter() {
        match button_event {
            ButtonAction::Clear => {
                commands.entity(map.0).despawn_recursive();
                commands.remove_resource::<MapEntity>();
                cell_map.clear();
                log::info!("regenerating map");
                spawn_map(&mut commands);
            }
            ButtonAction::SpawnConductor => {
                commands.insert_resource(SpawnSelection::Conductor);
            }
            ButtonAction::SpawnGenerator => {
                commands.insert_resource(SpawnSelection::default_generator());
            }
        }
    }
}
