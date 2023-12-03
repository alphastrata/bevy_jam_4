//! Shows how to render simple primitive shapes with a single color.
use bevy::prelude::*;
use camera::GameCamera;
use game::GamePlugin;
use menus::{mainmenu::MainMenuPlugin, splash::SplashPlugin};

/// Top-level states that the game can be in
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Splash, // TODO
    #[default]
    MainMenu,
    Playing,
    Paused,
}

/// Holding the current selection
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct PlayerState {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>() // Default state = Splash
        // add top-level plugins
        .add_plugins((SplashPlugin, MainMenuPlugin, GamePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}
