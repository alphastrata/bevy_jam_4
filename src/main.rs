//! Shows how to render simple primitive shapes with a single color.
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

use bgj::game::GamePlugin;
use bgj::menus::mainmenu::MainMenuPlugin;
use bgj::menus::splash::SplashPlugin;
use bgj::{camera::GameCamera, AppState};

/// Holding the current selection
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct PlayerState {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_state::<AppState>() // Default state = Splash
        // add top-level plugins
        .add_plugins((SplashPlugin, MainMenuPlugin, GamePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera::default()));
}
