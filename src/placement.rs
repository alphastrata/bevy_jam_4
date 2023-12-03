use bevy::{prelude::*, window::PrimaryWindow};

/// Spawn towers when clicked
pub struct TowerPlacementPlugin;
impl Plugin for TowerPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_at_click_pos);
    }
}

fn spawn_at_click_pos(
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_window.single();
}
