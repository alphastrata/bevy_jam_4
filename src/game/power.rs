//! Power system

use std::ops::ControlFlow;

use bevy::prelude::*;

use crate::{
    buildings::{core::TheCore, spawn_building, twr_custom_mats::TowerRadiusMaterial, Building},
    creeps::{CreepDie, SpawnCreep},
    AppState,
};

/// Marker component for buildings that require Power
#[derive(Component)]
pub struct RequiresPower;

/// The radius that a building supplies Power to
#[derive(Component)]
pub struct SupplyRadius(pub f32);
impl Default for SupplyRadius {
    fn default() -> Self {
        SupplyRadius(100.0)
    }
}

#[derive(Event)]
pub struct AddBuilding;

#[derive(Component)]
pub struct IsPowered;

#[derive(Resource, PartialEq)]
pub struct PowerDebug(pub bool);

pub struct PowerPlugin;
impl Plugin for PowerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerDebug(true));
        app.add_event::<AddBuilding>();
        app.add_systems(OnEnter(AppState::Gameplay), create_core);
        app.add_systems(
            Update,
            (update_powered_unpowered).run_if(in_state(AppState::Gameplay)),
        )
        .add_systems(
            PostUpdate,
            (update_powered_unpowered2).run_if(in_state(AppState::Gameplay)),
        )
        .add_systems(
            Update,
            (debug_power_map_ui).run_if(resource_exists_and_equals(PowerDebug(true))),
        );
    }
}

/// Spawn the [buildings::TheCore] building
fn create_core(
    mut commands: Commands,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<TowerRadiusMaterial>>,
    asset_server: Res<AssetServer>,
) {
    TheCore::custom_spawn(
        &mut commands,
        texture_atlases,
        meshes,
        materials,
        asset_server,
        Vec2::ZERO,
    );
}

pub fn update_powered_unpowered(
    mut commands: Commands,
    tower_spawned: EventReader<AddBuilding>,
    creep_spawned: EventReader<SpawnCreep>,
    creep_died: EventReader<CreepDie>,
    building_query: Query<(Entity, &Transform), With<RequiresPower>>,
    supply_query: Query<(Entity, &SupplyRadius, &Transform), With<IsPowered>>,
) {
    if !(creep_spawned.is_empty() && tower_spawned.is_empty() && creep_died.is_empty()) {
        // TODO: IF PERFORMANCE DIE, QUADTREE GO HERE.

        // for every building check that its powered by at least one building
        // O(n^2) ish (Power supplying buildings * power drawing buildings)
        building_query.iter().for_each(|(entity, drawer_tf)| {
            let is_powered = supply_query.iter().try_fold(
                None,
                |_acc: Option<f32>, (supply_entity, radius, supply_tf)| {
                    let drawer_pos = drawer_tf.translation;
                    let supply_pos = supply_tf.translation;

                    let distance = ((drawer_pos.x - supply_pos.x) * (drawer_pos.x - supply_pos.x)
                        + (drawer_pos.y - supply_pos.y) * (drawer_pos.y - supply_pos.y))
                        .sqrt();
                    if distance < radius.0 {
                        ControlFlow::Break(Some(supply_entity))
                    } else {
                        ControlFlow::Continue(None)
                    }
                },
            );
            match is_powered {
                ControlFlow::Break(is_powered) => {
                    if let Some(supply_ent) = is_powered {
                        // Only buildings other than this building itself can power it
                        if supply_ent != entity {
                            commands.entity(entity).insert(IsPowered);
                        } else {
                            commands.entity(entity).remove::<IsPowered>();
                        }
                    }
                }
                ControlFlow::Continue(_) => {
                    commands.entity(entity).remove::<IsPowered>();
                }
            }
        });
    }
}
/// Updates the set of towers that are powered or unpowered
pub fn update_powered_unpowered2(
    mut commands: Commands,
    tower_spawned: EventReader<AddBuilding>,
    creep_spawned: EventReader<SpawnCreep>,
    creep_died: EventReader<CreepDie>,
    building_query: Query<(Entity, &Transform), With<RequiresPower>>,
    supply_query: Query<(Entity, &SupplyRadius, &Transform), With<IsPowered>>,
) {
    if !(creep_spawned.is_empty() && tower_spawned.is_empty() && creep_died.is_empty()) {
        // TODO: IF PERFORMANCE DIE, QUADTREE GO HERE.

        // for every building check that its powered by at least one building
        // O(n^2) ish (Power supplying buildings * power drawing buildings)
        building_query.iter().for_each(|(entity, drawer_tf)| {
            let is_powered = supply_query.iter().try_fold(
                None,
                |_acc: Option<f32>, (supply_entity, radius, supply_tf)| {
                    let drawer_pos = drawer_tf.translation;
                    let supply_pos = supply_tf.translation;

                    let distance = ((drawer_pos.x - supply_pos.x) * (drawer_pos.x - supply_pos.x)
                        + (drawer_pos.y - supply_pos.y) * (drawer_pos.y - supply_pos.y))
                        .sqrt();
                    if distance < radius.0 {
                        ControlFlow::Break(Some(supply_entity))
                    } else {
                        ControlFlow::Continue(None)
                    }
                },
            );
            match is_powered {
                ControlFlow::Break(is_powered) => {
                    if let Some(supply_ent) = is_powered {
                        // Only buildings other than this building itself can power it
                        if supply_ent != entity {
                            commands.entity(entity).insert(IsPowered);
                        } else {
                            commands.entity(entity).remove::<IsPowered>();
                        }
                    }
                }
                ControlFlow::Continue(_) => {
                    commands.entity(entity).remove::<IsPowered>();
                }
            }
        });
    }
}

/// Render all the circles of power as transparent yellow circles
fn debug_power_map_ui(
    _q_supply: Query<(&SupplyRadius, &Transform)>,
    mut q_powered_buildings: Query<(&Transform, &mut TextureAtlasSprite), With<IsPowered>>,
    // mut q_unpowered_sprite_buildings: Query<(&Building, &Transform, &mut Sprite), Without<IsPowered>>,
    mut q_unpowered_spritesheet_buildings: Query<
        (&Building, &Transform, &mut TextureAtlasSprite),
        Without<IsPowered>,
    >,
) {
    q_powered_buildings
        .iter_mut()
        .for_each(|(_transform, mut sprite)| {
            sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
        });
    q_unpowered_spritesheet_buildings
        .iter_mut()
        .for_each(|(_, _transform, mut sprite)| {
            sprite.color = Color::rgba(0.5, 0.5, 0.5, 1.0);
        });
}
