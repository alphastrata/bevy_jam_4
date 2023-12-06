//! Shows how to render simple primitive shapes with a single color.
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, PrimaryWindow},
};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_tweening::TweeningPlugin;

use flora_cause::{
    components::button::ButtonPlugin,
    debug::display_debug::DisplayDebugPlugin,
    // debug::fps_counter::FPSPlugin,
    game::{camera::CameraState, keybinds::KeybindPlugin},
    scenes::{
        gameplay::GameplayPlugin, menu::MainMenuPlugin, pause::PausePlugin, splash::SplashPlugin,
    },
    AppState,
};

/// Holding the current selection
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct PlayerState {}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_plugins((
            TweeningPlugin,
            KeybindPlugin,
            DisplayDebugPlugin,
            ButtonPlugin,
        ))
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            default: Vec2::new(960.0, 640.0),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins((TweeningPlugin, KeybindPlugin))
        .add_plugins(TilemapPlugin)
        .add_state::<AppState>()
        .add_plugins((SplashPlugin, MainMenuPlugin, GameplayPlugin, PausePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    // unlocks fps with fast vsync (Presentation::Mailbox)
    let mut window = q_window.single_mut();
    window.present_mode = PresentMode::AutoNoVsync;
    #[cfg(not(target_arch = "wasm32"))]
    {
        window.present_mode = PresentMode::AutoNoVsync;
    }

    // window.present_mode = PresentMode::
    info!("{:?}", window.present_mode);

    commands.spawn((Camera2dBundle::default(), CameraState::default()));
}
