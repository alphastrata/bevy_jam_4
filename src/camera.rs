use bevy::prelude::*;

/// Play with this to modify the multiplier for camera pan movement
const PAN_SPEED: f32 = 5.0;

/// Component that adds our gameplay camera controls
#[derive(Component, Default)]
pub struct GameCamera;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pan_camera);
    }
}

fn pan_camera(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&GameCamera, &mut Transform, &OrthographicProjection)>,
) {
    let (_game_cam, mut transform, _proj) = query.single_mut();

    let mut direction_vecs = vec![];
    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
        direction_vecs.push(Vec3::Y)
    }
    if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
        direction_vecs.push(Vec3::NEG_Y)
    }
    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
        direction_vecs.push(Vec3::X)
    }
    if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
        direction_vecs.push(Vec3::NEG_X)
    }

    let camera_move_vector = direction_vecs
        .into_iter()
        .fold(Vec3::ZERO, |avg, vec| avg + vec)
        .try_normalize();

    if let Some(direction) = camera_move_vector {
        eprintln!("Camera move {direction}");
        debug_assert!(direction.z == 0.0);

        transform.translation += PAN_SPEED * direction;
        // WASD takes precedence over mouse dragging so early exit here
        return;
    }

    let _mouse_delta = Some(0.0); // TODO
}
