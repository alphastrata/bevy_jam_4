#![allow(clippy::type_complexity)]
#![allow(unused_imports, dead_code)]
use bevy::ecs::schedule::States;

pub mod camera;
pub mod game;
pub mod menus;
pub mod placement;
pub mod power;
pub mod towers;

/// Top-level states that the game can be in
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    Playing,
    Paused,
}
