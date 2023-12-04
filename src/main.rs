//! Shows how to render simple primitive shapes with a single color.
use bevy::{prelude::*, window::WindowResized};
use bevy_tweening::TweeningPlugin;

use flora_cause::{
    game::{camera::GameCamera, keybinds::KeybindPlugin},
    scenes::{gameplay::GameplayPlugin, mainmenu::MainMenuPlugin, splash::SplashPlugin},
    AppState,
};

/// Holding the current selection
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct PlayerState {}

fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            default: Vec2::new(960.0, 640.0),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins((TweeningPlugin, KeybindPlugin))
        .add_state::<AppState>() // Default state = Splash
        // add top-level plugins
        .add_plugins((SplashPlugin, MainMenuPlugin, GameplayPlugin))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (on_resize_system, toggle_resolution))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera::default()));
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;
/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    default: Vec2,
}

// Spawns the UI
fn setup_ui(mut cmd: Commands) {
    // Node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        // Text where we display current resolution
        root.spawn((
            TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font_size: 50.0,
                    ..default()
                },
            ),
            ResolutionText,
        ));
    });
}

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();
    if keys.just_pressed(KeyCode::Key1) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let res = resolution.default;
        window.resolution.set(res.x, res.y);
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.read() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}
