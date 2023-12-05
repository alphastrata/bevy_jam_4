#![allow(clippy::type_complexity)]
#![allow(unused_imports, dead_code)]
use bevy::ecs::{component::Component, schedule::States};

pub mod buildings;
pub mod creeps;
pub mod debug;
pub mod game;
pub mod scenes;

#[derive(Component)]
pub struct MovementSpeed(u32);

#[derive(Component)]
pub struct Targeting(usize); //TODO: Entity id, for .get()s of an 'n' of 1

#[derive(Component)]
pub struct AttackSpeed(usize);

#[derive(Component)]
pub struct Health(u32);

#[derive(Component)]
pub struct Experience(u32);

/// Top-level states that the game can be in
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    Playing,
    Paused,
}
