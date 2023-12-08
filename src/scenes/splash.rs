use std::time::Duration;

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_tweening::{lens::SpriteColorLens, Animator, Delay, EaseFunction, Tween};

use crate::{
    components::fade_transition::{transition_to, TransitionState},
    AppState,
};

const SPLASH_DURATION: f32 = 7.0;
const ZERO_ALPHA: Color = Color::rgba(1., 1., 1., 0.);
const MAX_ALPHA: Color = Color::rgba(1., 1., 1., 1.);

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Splash), show_splash)
            .add_systems(OnExit(AppState::Splash), exit_splash)
            .add_systems(Update, (keyboard_events).run_if(in_state(AppState::Splash)))
            .add_systems(Update, (tick_timer).run_if(in_state(AppState::Splash)));
    }
}

#[derive(Resource)]
struct SplashTimer(Timer);

#[derive(Component)]
struct OnSplashScreen;

fn keyboard_events(
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut transition_state: ResMut<TransitionState>,
) {
    #[allow(clippy::never_loop)] //TODO .read().next()
    if key.get_just_pressed().count() != 0 || mouse.get_just_pressed().count() != 0 {
        transition_to(AppState::MainMenu, &mut transition_state);
    }
}

fn tick_timer(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut transition_state: ResMut<TransitionState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        transition_to(AppState::MainMenu, &mut transition_state);
    }
}

/// Runs when we enter [AppState::Splash]
fn show_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let splash_handle = asset_server.load("textures/skeejy-no-background.png");
    let bubble_1_handle = asset_server.load("textures/bubble-1.png");
    let bubble_2_handle = asset_server.load("textures/bubble-2.png");
    let bubble_3_handle = asset_server.load("textures/bubble-3.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(512.0)),
                ..default()
            },
            texture: splash_handle,
            ..default()
        },
        OnSplashScreen,
    ));

    let duration = 2000;

    for (image, offset, delay_ms) in [
        (bubble_3_handle, Vec3::new(-225.0, 70.0, 0.0), 0),
        (bubble_2_handle, Vec3::new(-285.0, 100.0, 0.0), duration / 2),
        (bubble_1_handle, Vec3::new(-360.0, 150.0, 0.0), duration),
    ] {
        let untween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            SpriteColorLens {
                start: MAX_ALPHA,
                end: ZERO_ALPHA,
            },
        );
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            SpriteColorLens {
                start: ZERO_ALPHA,
                end: MAX_ALPHA,
            },
        );

        // repeat_strategy crash caused by this https://github.com/djeedai/bevy_tweening/issues/79
        let animator = if delay_ms > 0 {
            Animator::new(
                Delay::new(Duration::from_millis(delay_ms))
                    .then(tween)
                    .then(Delay::new(Duration::from_millis(duration)))
                    .then(untween),
            )
        } else {
            Animator::new(
                tween
                    .then(Delay::new(Duration::from_millis(duration)))
                    .then(untween),
            )
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    custom_size: Some(Vec2::splat(100.0)),
                    ..default()
                },
                texture: image,
                transform: Transform::from_translation(offset),
                ..default()
            },
            animator,
            OnSplashScreen,
        ));
    }

    commands.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
    commands.insert_resource(SplashTimer(Timer::from_seconds(
        SPLASH_DURATION,
        TimerMode::Once,
    )));
}

/// Runs when we exit [AppState::Splash]
fn exit_splash(nodes: Query<Entity, With<OnSplashScreen>>, mut commands: Commands) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
    commands.remove_resource::<SplashTimer>();
}
