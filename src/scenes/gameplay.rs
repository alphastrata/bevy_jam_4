use bevy::prelude::*;

use crate::{
    game::{
        camera::GameCameraPlugin, map::MapPlugin, placement::TowerPlacementPlugin, power::PowerPlugin,
        resources::ResourcePlugin,
    },
    AppState,
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
        ))
        .add_systems(OnEnter(AppState::Gameplay), capture_cursor)
        .add_systems(OnExit(AppState::Gameplay), release_cursor)
        .add_systems(Update, (toggle_pause).run_if(in_state(AppState::Gameplay)));
    }
}
