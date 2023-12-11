#![allow(clippy::type_complexity)]
#![allow(unused_imports, dead_code)]
use bevy::{
    ecs::{component::Component, schedule::States},
    prelude::{Deref, DerefMut, Event},
    time::Timer,
};

pub mod buildings;
pub mod creeps;
pub mod debug;
pub mod game;
pub mod global_systems;
pub mod scenes;

pub mod prelude {
    pub use crate::{AttackSpeed, CorpoPoints, Health, MovementSpeed, Tree};
}

/// Top-level states that the game can be in
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    Gameplay,
    DevScene,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PauseMenuState {
    #[default]
    Unpaused,
    Paused,
}

const TERRAIN_Z: f32 = 0.1;
const SHADER_Z: f32 = -0.05;
const BUILDING_Z: f32 = 0.3;
const CREEP_Z: f32 = 9.0;
const HP_BAR_Z: f32 = 0.4;
const PLACEMENT_Z: f32 = 0.5;

#[derive(Event)]
pub struct GameOver;

/// Marker component indicating an entity needs to be torn down (destroyed) when going
/// from [AppState::Gameplay] or [AppState::Paused] back to [AppState::MainMenu]
#[derive(Component)]
pub struct Teardown;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct MovementSpeed(u32);

#[derive(Component)]
pub struct Targeting(usize); //TODO: Entity id, for .get()s of an 'n' of 1

#[derive(Component)]
pub struct AttackSpeed(usize);

#[derive(Component)]
pub struct Health(u32);

impl Health {
    pub fn deduct(&mut self, value: u32) {
        self.0 = self.0.saturating_sub(value);
    }
}

/// The game's currency
#[derive(Component)]
pub struct CorpoPoints(u32);

/// The range/radius a given building/creep can effect
#[derive(Component)]
pub struct Range(u32);

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
