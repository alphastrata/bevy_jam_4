use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use crate::{
    buildings::drain::DrainTowerPlugin,
    creeps::CreepPlugin,
    game::{
        camera::GameCameraPlugin, map::MapPlugin, placement::TowerPlacementPlugin,
        power::PowerPlugin, resources::ResourcePlugin,
    },
    AppState,
};

use super::pause::{capture_cursor, pause, release_cursor, PausePlugin};

/// Defines systems that should run when in the [AppState::Playing] State
pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TilemapPlugin,
            CreepPlugin,
            GameCameraPlugin,
            MapPlugin,
            TowerPlacementPlugin,
            PowerPlugin,
            ResourcePlugin,
            DrainTowerPlugin,
        ))
        .add_systems(OnEnter(AppState::Gameplay), capture_cursor)
        .add_systems(Update, (pause).run_if(in_state(AppState::Gameplay)));
    }
}
