use super::{twr_custom_mats::TowerRadiusMaterial, Building, BuildingDefinition, BuildingState};
use crate::{
    game::power::{IsPowered, RequiresPower, SupplyRadius},
    AnimationIndices, AnimationTimer, AppState, Health, Teardown, BUILDING_Z, SHADER_Z,
};
use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use std::path::Path;

const BUILDING_ANIM: AnimationIndices = AnimationIndices { first: 1, last: 11 };
const ACTIVE_ANIM: AnimationIndices = AnimationIndices {
    first: 12,
    last: 22,
};

/// Distributes power to other buildings within a radius.
#[derive(Component, Default)]
pub struct DistributionTower;
impl BuildingDefinition for DistributionTower {
    const SPRITE_PATH: &'static str = "textures/tower.png";
    const BASE_HEALTH: u32 = 100;
    const COST: u32 = 300;
    const BUILD_TIME: u32 = 6;
    const NAME: &'static str = "Distribution Tower";
    const DESCRIPTION: &'static str = "";

    fn add_extra_components(commands: &mut Commands, ent_id: Entity) {
        commands.entity(ent_id).insert(SupplyRadius(300.0));
    }
}

impl DistributionTower {
    pub fn custom_spawn(
        commands: &mut Commands,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<TowerRadiusMaterial>>,
        asset_server: Res<AssetServer>,
        pos: Vec2,
    ) -> Entity {
        let texture_handle = asset_server.load(Self::SPRITE_PATH);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 64.0), 23, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let radius_display = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(300.).into()).into(),
                material: materials.add(TowerRadiusMaterial {
                    color: Color::rgb(0.9453125, 0.0, 0.0625),
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, SHADER_Z)),
                ..default()
            })
            .id();

        let ent_id = commands
            .spawn((
                DistributionTower,
                Building,
                BuildingState::Building,
                RequiresPower,
                Teardown,
                Health(Self::BASE_HEALTH),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(BUILDING_ANIM.first),
                    transform: Transform::from_translation(Vec3::new(pos.x, pos.y, BUILDING_Z)),
                    ..default()
                },
                BUILDING_ANIM,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ))
            .add_child(radius_display)
            .id();
        Self::add_extra_components(commands, ent_id);

        ent_id
    }
}

pub struct DistributionTowerPlugin;
impl Plugin for DistributionTowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_sprite).run_if(in_state(AppState::Gameplay)),
        );
    }
}

fn animate_sprite(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut BuildingState,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<DistributionTower>,
    >,
    time: Res<Time>,
) {
    for (ent, mut state, indices, mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            sprite.index = if sprite.index == indices.last {
                if *state == BuildingState::Building {
                    info!("Distribution tower finished building.");

                    commands.entity(ent).remove::<AnimationIndices>();
                    commands.entity(ent).insert(ACTIVE_ANIM);
                    *state = BuildingState::Active;
                    break;
                }
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
