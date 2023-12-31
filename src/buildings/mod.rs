use bevy::prelude::*;
use std::path::Path;

use self::twr_custom_mats::TowerRadiusMaterial;
use self::{
    distribution::DistributionTower,
    drain::{DrainTower, DrainTowerPlugin},
    radar::RadarTower,
};
use crate::global_systems::eargasm::AudioRequest;
use crate::{game::hp_bars::HpBarUISettings, Health};
use crate::{Teardown, BUILDING_Z};

pub mod core;
pub mod distribution;
pub mod twr_custom_mats {

    use bevy::{
        prelude::*,
        render::render_resource::{AsBindGroup, ShaderRef},
        sprite::Material2d,
    };

    #[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
    pub struct TowerRadiusMaterial {
        #[uniform(0)]
        pub color: Color,
    }

    impl Material2d for TowerRadiusMaterial {
        fn fragment_shader() -> ShaderRef {
            "shaders/tower_radius.wgsl".into()
        }
    }
}
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
    hp_bar: HpBarUISettings,
    sprite: SpriteBundle,
}

#[derive(Component, PartialEq)]
pub enum BuildingState {
    Building,
    Active,
    Inactive,
}

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
        .spawn((
            MinimalBuilding {
                marker: Building,
                health: Health(B::BASE_HEALTH),
                hp_bar: HpBarUISettings {
                    max: B::BASE_HEALTH,
                    offset: None,
                },
                sprite: SpriteBundle {
                    texture: sprite_texture,
                    transform: Transform::from_translation(Vec3::new(pos.x, pos.y, BUILDING_Z)),
                    ..default()
                },
            },
            Teardown,
        ))
        .id();

    B::add_extra_components(commands, ent_id);

    ent_id
}

// /// Representing the types of buildings we have
#[derive(Clone, Hash, Component, Debug, PartialEq, Eq)]
pub enum BuildingType {
    Radar,
    /// Pylons, you must construct additional...
    Distribution,
    Drain,
}

impl BuildingType {
    #[allow(clippy::too_many_arguments)]
    pub fn spawn(
        &self,
        commands: &mut Commands,
        texture_atlases: ResMut<Assets<TextureAtlas>>,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<TowerRadiusMaterial>>,
        asset_server: Res<AssetServer>,
        pos: Vec2,
        mut audio_mngr: EventWriter<AudioRequest>,
    ) {
        match self {
            BuildingType::Radar => {
                audio_mngr.send(AudioRequest {
                    component: crate::global_systems::eargasm::AudioComponent::Radar1(
                        crate::global_systems::eargasm::Radar1,
                    ),
                });
                spawn_building::<RadarTower>(commands, asset_server, pos)
            }
            BuildingType::Distribution => {
                audio_mngr.send(AudioRequest {
                    component: crate::global_systems::eargasm::AudioComponent::Electric(
                        crate::global_systems::eargasm::Electric,
                    ),
                });

                DistributionTower::custom_spawn(
                    commands,
                    texture_atlases,
                    meshes,
                    materials,
                    asset_server,
                    pos,
                )
            }
            BuildingType::Drain => {
                audio_mngr.send(AudioRequest {
                    component: crate::global_systems::eargasm::AudioComponent::Thump(
                        crate::global_systems::eargasm::Thump,
                    ),
                });

                DrainTower::custom_spawn(
                    commands,
                    texture_atlases,
                    meshes,
                    materials,
                    asset_server,
                    pos,
                )
            }
        };
    }

    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::Distribution => DistributionTower::COST,
            BuildingType::Radar => RadarTower::COST,
            BuildingType::Drain => RadarTower::COST,
        }
    }

    pub fn sprite(&self) -> &'static str {
        match self {
            BuildingType::Radar => "",
            BuildingType::Distribution => "textures/tower_single.png",
            BuildingType::Drain => "textures/sucky-uppy-single-frame.png",
        }
    }
}
