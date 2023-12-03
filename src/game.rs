use bevy::prelude::*;

use crate::{camera::GameCameraPlugin, placement::TowerPlacementPlugin};

/// Defines systems that should run when in the [AppState::Playing] State
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCameraPlugin, TowerPlacementPlugin));
    }
}
