use bevy::prelude::*;

use crate::game::{camera::GameCameraPlugin, placement::TowerPlacementPlugin, power::PowerPlugin};

/// Defines systems that should run when in the [AppState::Playing] State
pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((GameCameraPlugin, TowerPlacementPlugin, PowerPlugin));
	}
}