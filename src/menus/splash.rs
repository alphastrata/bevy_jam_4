use bevy::prelude::*;

use crate::AppState;

const SPLASH_DURATION: f32 = 2.0;

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
        println!("Splash timer elapsed. Proceed to MainMenu");
        app_state.set(AppState::MainMenu);
    }
}

/// Runs when we enter [AppState::Splash]
fn show_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("textures/skeejy-no-background.png");

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
                image: UiImage::new(texture_handle),
                ..default()
            });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(SPLASH_DURATION, TimerMode::Once)));
    commands.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
}

/// Runs when we exit [AppState::Splash]
fn exit_splash(nodes: Query<Entity, With<OnSplashScreen>>, mut commands: Commands) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}
