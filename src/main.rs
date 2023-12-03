//! Shows how to render simple primitive shapes with a single color.

use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use camera::GameCamera;
use game::GamePlugin;
use menus::{mainmenu::MainMenuPlugin, splash::SplashPlugin};

mod camera;
mod game;
mod menus;
mod placement;
mod towers;

/// Top-level states that the game can be in
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    Playing,
    Paused,
}

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
    commands.spawn((Camera2dBundle::default(), GameCamera));

    // left as a reference for drawing shapes
    // // Circle
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
    //     ..default()
    // });
    //
    // // Rectangle
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(50.0, 100.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
    //     ..default()
    // });
    //
    // // Quad
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes
    //         .add(shape::Quad::new(Vec2::new(50., 100.)).into())
    //         .into(),
    //     material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
    //     transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
    //     ..default()
    // });
}
