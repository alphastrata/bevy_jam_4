use bevy::prelude::*;

use crate::{camera::GameCameraPlugin, placement::TowerPlacementPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCameraPlugin, TowerPlacementPlugin));
    }
}
