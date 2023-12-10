use bevy::prelude::*;

use crate::{
    buildings::{distribution::DistributionTowerPlugin, drain::DrainTowerPlugin},
    game::{
        camera::GameCameraPlugin, hp_bars::HealthBarUIPlugin, map::MapPlugin,
        placement::TowerPlacementPlugin, power::PowerPlugin, resources::ResourcePlugin,
    },
    AppState, Teardown,
};

use super::pause::{capture_cursor, release_cursor, toggle_pause, PausePlugin};

/// Defines systems that should run when in the [AppState::Playing] State
pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
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
        .add_systems(OnExit(AppState::Gameplay), release_cursor)
        .add_systems(
            OnTransition {
                from: AppState::Gameplay,
                to: AppState::MainMenu,
            },
            teardown_all,
        )
        .add_systems(
            OnTransition {
                from: AppState::Paused,
                to: AppState::MainMenu,
            },
            teardown_all,
        )
        .add_systems(Update, (toggle_pause).run_if(in_state(AppState::Gameplay)));
    }
}

fn teardown_all(mut commands: Commands, to_teardown: Query<(Entity, &Teardown)>) {
    info!("Tearing down all gameplay entities!!!");
    to_teardown.iter().for_each(|(entity, _)| {
        commands.entity(entity).despawn_recursive();
    });
}
