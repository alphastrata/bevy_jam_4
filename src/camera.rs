use bevy::prelude::*;

/// Component that adds our gameplay camera controls
#[derive(Component, Default)]
pub struct GameCamera;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (pan_camera, zoom_camera));
    }
}

fn zoom_camera() {
    println!("zoom camera system");
}

fn pan_camera() {
    println!("pan camera system");
}
