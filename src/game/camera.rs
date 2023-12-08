use bevy::{
    core::Zeroable,
    core_pipeline::clear_color::{self, ClearColorConfig},
    input::mouse::{MouseMotion, MouseWheel},
    log,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::{Layer, RenderLayers},
    },
    window::{PrimaryWindow, WindowRef},
};
use bevy_tweening::Lerp;

use crate::AppState;

use super::keybinds::FloraCommand;

// how close to the edges of the screen before camera moves
const PAN_THRESHOLD: Vec2 = Vec2::splat(64.0);

// loops camera when x value is met, stops camera from progressing when y value is met
const CAMERA_BOUNDS_MIN: Vec2 = Vec2::new(-2000.0, -2000.0);
// loops camera when x value is met, stops camera from progressing when y value is met
const CAMERA_BOUNDS_MAX: Vec2 = Vec2::new(2000.0, 2000.0);

// current camera velocity is multiplied by this value to slow it down
const FRICTION: Vec2 = Vec2::splat(0.90);
// max accel to speed up the camera to
const ACCELERATION: Vec2 = Vec2::splat(100.0);
// max speed the camera can travel
const VELOCITY_MAX: Vec2 = Vec2::splat(1000.0);
// arbitrary value to match zoom and drag
const DRAG_FACTOR: Vec2 = Vec2::splat(0.5);

// how fast to zoom when px value returned from event
const ZOOM_VELOCITY_PX: f32 = 0.2;
// how fast to zoom when line value returned from event
const ZOOM_VELOCITY_LINE: f32 = 2.0;
// how fast to lerp to result
const ZOOM_FACTOR: f32 = 0.2;
// how far in are we allowed to zoom
const ZOOM_MAX: f32 = 0.5;
// how far out are we allowed to zoom
const ZOOM_MIN: f32 = 128.0;

pub const UI_LAYER: Layer = 1;
pub const V3D_LAYER: Layer = 1;

pub fn ui_layer() -> RenderLayers {
    RenderLayers::layer(UI_LAYER)
}

pub fn v3d_layer() -> RenderLayers {
    RenderLayers::layer(V3D_LAYER)
}

/// Component that adds our gameplay camera controls
#[derive(Component)]
pub struct CameraState {
    zoom_target: f32,
    velocity: Vec2,
    prev_mouse_pos: Vec2,
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            zoom_target: 1.0,
            velocity: Vec2::ZERO,
            prev_mouse_pos: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct CameraState3d;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (move_camera).run_if(in_state(AppState::Gameplay)));
    }
}

#[derive(Component)]
pub struct ViewCamera;

impl Default for ViewCamera {
    fn default() -> Self {
        ViewCamera {}
    }
}

#[derive(Component)]
pub struct V3DCamera;
impl Default for V3DCamera {
    fn default() -> Self {
        V3DCamera {}
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 128,
        height: 128,
        ..default()
    };
    let v3d_size = Extent3d {
        width: 960,
        height: 640,
        ..default()
    };

    let mut target_img = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    target_img.resize(size);

    let target_handle = images.add(target_img);

    let mut v3d_target_img = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: v3d_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    v3d_target_img.resize(v3d_size);

    let v3d_target_handle = images.add(v3d_target_img);

    let ui_image = commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    ..default()
                },
                image: UiImage::new(target_handle.clone()),
                z_index: ZIndex::Global(i32::MIN),
                visibility: Visibility::Hidden,
                ..default()
            },
            ui_layer(),
        ))
        .id();

    let v3d_img = commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    ..default()
                },
                image: UiImage::new(v3d_target_handle.clone()),
                z_index: ZIndex::Global(i32::MAX),
                visibility: Visibility::Hidden,
                ..default()
            },
            ui_layer(),
        ))
        .id();

    let game_cam = commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(target_handle.clone()),
                    ..default()
                },
                ..default()
            },
            UiCameraConfig { show_ui: false },
            CameraState::default(),
        ))
        .id();

    let v3d_cam = commands
        .spawn((
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::PINK),
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(v3d_target_handle.clone()),
                    ..default()
                },
                ..default()
            },
            v3d_layer(),
            UiCameraConfig { show_ui: false },
            V3DCamera::default(),
        ))
        .id();

    let view_cam = commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    target: RenderTarget::Window(WindowRef::Primary),
                    ..default()
                },
                ..default()
            },
            ui_layer(),
            ViewCamera::default(),
        ))
        .id();
}

fn move_camera(
    input: Res<Input<FloraCommand>>,
    time: Res<Time>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(
        &mut CameraState,
        &mut Transform,
        &mut OrthographicProjection,
    )>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_ev: EventReader<MouseMotion>,
    mut wheel_ev: EventReader<MouseWheel>,
) {
    let (mut state, mut transform, mut projection) = query.single_mut();

    // debug reset
    if input.pressed(FloraCommand::ResetCamera) {
        state.zoom_target = CameraState::default().zoom_target;
        state.velocity = CameraState::default().velocity;

        transform.translation = Vec3::ZERO;
        projection.scale = CameraState::default().zoom_target;
    }

    // zoom
    use bevy::input::mouse::MouseScrollUnit;
    for ev in wheel_ev.read() {
        state.zoom_target += match ev.unit {
            MouseScrollUnit::Pixel => ev.y * ZOOM_VELOCITY_PX,
            MouseScrollUnit::Line => ev.y * ZOOM_VELOCITY_LINE,
        }
    }
    state.zoom_target = state.zoom_target.clamp(ZOOM_MAX, ZOOM_MIN);
    projection.scale = projection.scale.lerp(&state.zoom_target, &ZOOM_FACTOR);

    // read the mouse motion or it builds up speed
    let mut mousetache = Vec2::ZERO;
    for ev in mouse_ev.read() {
        if mouse_input.pressed(MouseButton::Middle) {
            mousetache += ev.delta;
        }
    }

    // if there's middle mouse down + mouse motion, then move the camera, ignore other inputs at this point
    if mousetache != Vec2::ZERO {
        mousetache.x *= -1.0;
        mousetache *= DRAG_FACTOR * projection.scale;
        transform.translation += Vec3::new(mousetache.x, mousetache.y, 0.0);
        state.velocity = mousetache;
    }

    // quit out of other motion of middle click to drag is being used
    if mouse_input.pressed(MouseButton::Middle) {
        return;
    }

    // keyboard navigation
    let mut accel = Vec2::ZERO;
    if input.pressed(FloraCommand::Left) {
        accel -= Vec2::X;
    }
    if input.pressed(FloraCommand::Right) {
        accel += Vec2::X;
    }
    if input.pressed(FloraCommand::Up) {
        accel += Vec2::Y;
    }
    if input.pressed(FloraCommand::Down) {
        accel -= Vec2::Y;
    }

    let window = q_window.single();
    // edge pan
    let mut pan = Vec2::ZERO;
    let top_left = Vec2::ZERO;
    let bot_right = Vec2::new(window.width(), window.height());

    // if we're in the pan threshold
    if let Some(cursor_position) = window.cursor_position() {
        // if we're not in the threshold, try the other side
        pan.x = ((cursor_position.x - (bot_right.x - PAN_THRESHOLD.x)) / PAN_THRESHOLD.x).min(1.0);
        if pan.x < 0.0 {
            pan.x =
                ((cursor_position.x - (top_left.x + PAN_THRESHOLD.x)) / PAN_THRESHOLD.x).max(-1.0);
            if pan.x > 0.0 {
                pan.x = 0.0;
            }
        }
        // if we're in the pan threshold
        pan.y = ((cursor_position.y - (bot_right.y - PAN_THRESHOLD.y)) / PAN_THRESHOLD.y).min(1.0);
        // if we're not in the threshold, try the other side
        if pan.y < 0.0 {
            pan.y =
                ((cursor_position.y - (top_left.y + PAN_THRESHOLD.y)) / PAN_THRESHOLD.y).max(-1.0);
            if pan.y > 0.0 {
                pan.y = 0.0;
            }
        }
    }

    pan *= Vec2::new(1.0, -1.0);
    accel += pan;

    // motion physics
    accel = accel.normalize_or_zero();
    state.velocity += accel * Vec2::splat(time.delta_seconds()) * ACCELERATION * projection.scale;
    state.velocity = state.velocity.clamp(
        -VELOCITY_MAX * projection.scale,
        VELOCITY_MAX * projection.scale,
    );

    transform.translation += Vec3::from((state.velocity, 0.0));
    state.velocity *= FRICTION;
}
