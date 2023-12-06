use crate::{
    buildings::{distribution::DistributionTower, spawn_building},
    game::{camera::CameraState, power::AddBuilding},
    AppState,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_mod_picking::prelude::*;

/// Spawn towers when clicked
pub struct TowerPlacementPlugin;
impl Plugin for TowerPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);

        app.add_systems(
            Update,
            (spawn_at_click_pos).run_if(in_state(AppState::Gameplay)),
        );
    }
}

fn spawn_at_click_pos(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraState>>,
    mouse_btns: Res<Input<MouseButton>>,
    mut add_building: EventWriter<AddBuilding>,
) {
    if mouse_btns.just_pressed(MouseButton::Right) {
        let window = q_window.single();
        let (camera, camera_transform) = q_camera.single();

        // convert viewport pos to worldspace
        if let Some(world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            let _ = spawn_building::<DistributionTower>(&mut commands, asset_server, world_pos);
            add_building.send(AddBuilding);
        }
    }
}
