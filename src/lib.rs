#![allow(clippy::type_complexity)]
#![allow(unused_imports, dead_code)]
use bevy::ecs::schedule::States;

pub mod buildings;
pub mod game;
pub mod scenes;

/// Top-level states that the game can be in
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
	#[default]
	Splash,
	MainMenu,
	Playing,
	Paused,
}
