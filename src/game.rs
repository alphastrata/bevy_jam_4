use bevy::prelude::*;

use crate::gpu_entities::setup_instances;
use crate::gpu_entities::GpuInstancingPlugin;
use crate::{camera::GameCameraPlugin, placement::TowerPlacementPlugin};

/// Defines systems that should run when in the [AppState::Playing] State
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCameraPlugin, TowerPlacementPlugin));
    }
}
