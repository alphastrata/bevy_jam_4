use crate::{camera::GameCamera, towers::spawn_fire_tower, AppState};
use bevy::{math::vec4, prelude::*};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;

/// Spawn towers when clicked
pub struct TowerPlacementPlugin;
impl Plugin for TowerPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);

        app.add_systems(
            Update,
            (spawn_at_click_pos).run_if(in_state(AppState::Playing)),
        );
    }
}

fn spawn_at_click_pos(
    commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mouse_btns: Res<Input<MouseButton>>,
) {
    if mouse_btns.just_pressed(MouseButton::Right) {
        let window = q_window.single();
        let (camera, camera_transform) = q_camera.single();

        // convert viewport pos to worldspace
        if let Some(world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            spawn_fire_tower(commands, world_pos);
        }
    }
}
