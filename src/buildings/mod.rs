use bevy::prelude::*;
use std::path::Path;

use crate::Health;

use self::{distribution::DistributionTower, radar::RadarTower};

pub mod core;
pub mod distribution;
pub mod drain;
pub mod radar;

/// Marker component all buildings should have
#[derive(Component)]
pub struct Building;

/// Blueprint for a generic tower entity
#[derive(Bundle)]
pub struct MinimalBuilding {
    marker: Building,
    health: Health,
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Experience(u32);

/// Common definitions needed to have a building
pub trait BuildingDefinition: Default {
    const SPRITE_PATH: &'static str;
    const BASE_HEALTH: u32;
    /// How many Corporation Points it costs to build
    const COST: u32;
    /// How long it takes to build in seconds
    const BUILD_TIME: u32;
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    fn add_extra_components(commands: &mut Commands, end_id: Entity);
}

pub fn spawn_building<B: BuildingDefinition>(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    pos: Vec2,
) -> Entity {
    let sprite_texture = asset_server.load(B::SPRITE_PATH);

    let ent_id = commands
        .spawn(MinimalBuilding {
            marker: Building,
            health: Health(B::BASE_HEALTH),
            sprite: SpriteBundle {
                texture: sprite_texture,
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.10)),
                ..default()
            },
        })
        .id();

    B::add_extra_components(commands, ent_id);

    ent_id
}

// /// Representing the types of buildings we have
#[derive(Clone, Hash, Component, Debug, PartialEq, Eq)]
pub enum BuildingType {
    /// Combats wind
    Fan,
    /// tbd
    Shield,
    /// tbd
    Radar,
    /// tbd
    Doppler,
    /// Pylons, you must construct additional...
    Distribution,
    ///
    Roboport,
    ///
    LoggingCentre,
}

impl BuildingType {
    pub fn spawn(&self, commands: &mut Commands, asset_server: Res<AssetServer>, pos: Vec2) {
        match self {
            BuildingType::Fan => todo!(),
            BuildingType::Shield => todo!(),
            BuildingType::Radar => spawn_building::<RadarTower>(commands, asset_server, pos),
            BuildingType::Doppler => todo!(),
            BuildingType::Distribution => {
                spawn_building::<DistributionTower>(commands, asset_server, pos)
            }
            BuildingType::Roboport => todo!(),
            BuildingType::LoggingCentre => todo!(),
        };
    }

    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::Distribution => DistributionTower::COST,
            BuildingType::Radar => RadarTower::COST,
            _ => 0,
        }
    }
}
