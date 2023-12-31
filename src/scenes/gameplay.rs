use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_ecs_tilemap::TilemapPlugin;

use crate::{
    buildings::{
        core::TheCorePlugin, distribution::DistributionTowerPlugin, drain::DrainTowerPlugin,
        twr_custom_mats::TowerRadiusMaterial, Building,
    },
    creeps::CreepPlugin,
    game::{
        camera::GameCameraPlugin, depletion::DepletionPlugin, hp_bars::HealthBarUIPlugin,
        hud::HudPlugin, map::MapPlugin, placement::TowerPlacementPlugin, power::PowerPlugin,
        resources::ResourcePlugin,
    },
    AppState, Teardown,
};

use super::pause::{capture_cursor, check_for_keyboard_pause, release_cursor, PausePlugin};

/// Defines systems that should run when in the [AppState::Playing] State
pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DepletionPlugin,
            TilemapPlugin,
            CreepPlugin,
            GameCameraPlugin,
            MapPlugin,
            TowerPlacementPlugin,
            PowerPlugin,
            ResourcePlugin,
            HealthBarUIPlugin,
            TheCorePlugin,
            DrainTowerPlugin,
            DistributionTowerPlugin,
            HudPlugin,
            Material2dPlugin::<TowerRadiusMaterial>::default(),
        ))
        .add_systems(OnEnter(AppState::Gameplay), capture_cursor)
        .add_systems(
            Update,
            (check_for_keyboard_pause).run_if(in_state(AppState::Gameplay)),
        )
        .add_systems(OnExit(AppState::Gameplay), (teardown_all, release_cursor));
    }
}

fn teardown_all(mut commands: Commands, to_teardown: Query<Entity, With<Teardown>>) {
    info!("Tearing down all gameplay entities!!!");
    to_teardown.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
