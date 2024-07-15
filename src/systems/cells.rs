use crate::components::CellBundle;
use crate::events::{ClearWorld, RemoveCell, SpawnCell};
use crate::resources::{BoardMaterials, MapEntity};
use crate::CELL_SIZE;
use bevy::log;
use bevy::prelude::*;
use bevy_life::{CellMap, MooreCell2d, WireWorldCellState};

pub fn setup(mut commands: Commands) {
    commands.insert_resource(BoardMaterials::new());
    // map
    spawn_map(&mut commands);
}

pub fn spawn_map(commands: &mut Commands) {
    let map_size = 5;

    let entity = commands
        .spawn(SpatialBundle::default())
        .with_children(|builder| {
            for y in -map_size..=map_size {
                for x in -map_size..=map_size {
                    if (x > -map_size && x < map_size && y > -map_size && y < map_size)
                        || ((x == -map_size || x == map_size) && (y == -map_size || y == map_size))
                    {
                        continue;
                    }
                    let state = if x == 0 && y == -map_size {
                        WireWorldCellState::ElectronTail
                    } else if x == 1 && y == -map_size {
                        WireWorldCellState::ElectronHead
                    } else {
                        WireWorldCellState::Conductor
                    };
                    builder.spawn(CellBundle {
                        state,
                        ..CellBundle::new(IVec2::new(x, y), CELL_SIZE)
                    });
                }
            }
        })
        .id();
    commands.insert_resource(MapEntity(entity));
    log::info!("Map generated");
}

pub fn spawn_cells(
    mut commands: Commands,
    mut spawn_cells_evr: EventReader<SpawnCell>,
    map: Res<MapEntity>,
    cell_map: Res<CellMap<MooreCell2d>>,
) {
    for event in spawn_cells_evr.read() {
        if cell_map.get_cell(&event.pos).is_some() {
            continue;
        }
        log::info!("Spawning new cell at {:?}", event.pos,);
        let mut cmd = commands.spawn(CellBundle::new(event.pos, CELL_SIZE));
        if let Some(generator) = &event.generator {
            cmd.insert(generator.clone());
        }
        cmd.set_parent(map.0);
    }
}

pub fn remove_cells(
    mut commands: Commands,
    mut remove_cells_evr: EventReader<RemoveCell>,
    cell_map: Res<CellMap<MooreCell2d>>,
) {
    for event in remove_cells_evr.read() {
        if let Some(entity) = cell_map.get_cell(&event.pos) {
            log::info!("Deleting cell at {:?}", event.pos);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn clear_map(
    mut commands: Commands,
    mut clear_map_evr: EventReader<ClearWorld>,
    map: Res<MapEntity>,
) {
    if clear_map_evr.is_empty() {
        return;
    }
    log::info!("Clearing Map");
    clear_map_evr.clear();
    commands.entity(map.0).despawn_descendants();
}
