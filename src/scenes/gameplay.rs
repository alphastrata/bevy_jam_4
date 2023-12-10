use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use crate::{
    buildings::{distribution::DistributionTowerPlugin, drain::DrainTowerPlugin},
    creeps::CreepPlugin,
    game::{
        camera::GameCameraPlugin, hp_bars::HealthBarUIPlugin, map::MapPlugin,
        placement::TowerPlacementPlugin, power::PowerPlugin, resources::ResourcePlugin,
    },
    AppState, Teardown,
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
            HealthBarUIPlugin,
            DrainTowerPlugin,
            DistributionTowerPlugin,
        ))
        .add_systems(OnEnter(AppState::Gameplay), capture_cursor)
        .add_systems(Update, (pause).run_if(in_state(AppState::Gameplay)));
    }
}

fn teardown_all(mut commands: Commands, to_teardown: Query<(Entity, &Teardown)>) {
    info!("Tearing down all gameplay entities!!!");
    to_teardown.iter().for_each(|(entity, _)| {
        commands.entity(entity).despawn_recursive();
    });
}
