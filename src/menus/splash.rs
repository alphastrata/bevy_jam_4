use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::SpriteColorLens, Animator, Delay, EaseFunction, Tween};

use crate::AppState;

const SPLASH_DURATION: f32 = 4.0;
const ZERO_ALPHA: Color = Color::rgba(1., 1., 1., 0.);
const MAX_ALPHA: Color = Color::rgba(1., 1., 1., 1.);

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Splash), show_splash)
            .add_systems(OnExit(AppState::Splash), exit_splash)
            .add_systems(Update, (tick_timer).run_if(in_state(AppState::Splash)));
    }
}

#[derive(Resource)]
struct SplashTimer(Timer);

#[derive(Component)]
struct OnSplashScreen;

fn tick_timer(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        info!("Splash timer elapsed. Proceed to MainMenu");

        app_state.set(AppState::MainMenu);
    }
}

/// Runs when we enter [AppState::Splash]
fn show_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let splash_handle = asset_server.load("textures/skeejy-no-background.png");
    let bubble_1_handle = asset_server.load("textures/bubble-1.png");
    let bubble_2_handle = asset_server.load("textures/bubble-2.png");
    let bubble_3_handle = asset_server.load("textures/bubble-3.png");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Vw(100.),
                    ..default()
                },
                image: UiImage::new(splash_handle),
                ..default()
            });
        });

    for (image, offset, delay_ms) in [
        (bubble_3_handle, 300.0, 0),
        (bubble_2_handle, 400.0, 1000),
        (bubble_1_handle, 550.0, 2000),
    ] {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            SpriteColorLens {
                start: ZERO_ALPHA,
                end: MAX_ALPHA,
            },
        );
        // FIXME: Get this to work without panicking. It should fade back to transparent
        // from opaque.
        // .with_repeat_count(RepeatCount::Finite(2))
        // .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

        let animator = if delay_ms > 0 {
            let delay = Delay::new(Duration::from_millis(delay_ms));
            Animator::new(delay.then(tween))
        } else {
            Animator::new(tween)
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    ..default()
                },
                texture: image,
                transform: Transform::from_translation(Vec3::new(-offset, offset, 0.0)),
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
