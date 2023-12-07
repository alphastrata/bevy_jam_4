//! Power system

use std::ops::ControlFlow;

use bevy::prelude::*;

use crate::{
    buildings::{core::TheCore, spawn_building, Building},
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
            PostUpdate,
            (update_powered_unpowered).run_if(in_state(AppState::Gameplay)),
        )
        .add_systems(
            Update,
            (debug_power_map_ui).run_if(resource_exists_and_equals(PowerDebug(true))),
        );
    }
}

/// Spawn the [buildings::Core] building
fn create_core(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_building::<TheCore>(&mut commands, asset_server, Vec2::ZERO);
}

/// Updates the set of towers that are powered or unpowered
fn update_powered_unpowered(
    mut commands: Commands,
    mut update_trigger: EventReader<AddBuilding>,
    building_query: Query<(Entity, &Transform), With<RequiresPower>>,
    supply_query: Query<(&SupplyRadius, &Transform)>,
) {
    if update_trigger.read().last().is_some() {
        // TODO: IF PERFORMANCE DIE, QUADTREE GO HERE.

        // for every building check that its powered by at least one building
        // O(n^2) ish (Power supplying buildings * power drawing buildings)
        building_query.iter().for_each(|(entity, drawer_tf)| {
            let is_powered =
                supply_query
                    .iter()
                    .try_fold(None, |_acc: Option<f32>, (radius, supply_tf)| {
                        let drawer_pos = drawer_tf.translation;
                        let supply_pos = supply_tf.translation;

                        let distance = ((drawer_pos.x - supply_pos.x)
                            * (drawer_pos.x - supply_pos.x)
                            + (drawer_pos.y - supply_pos.y) * (drawer_pos.y - supply_pos.y))
                            .sqrt();
                        if distance < radius.0 {
                            ControlFlow::Break(Some(radius.0))
                        } else {
                            ControlFlow::Continue(None)
                        }
                    });
            if is_powered.is_break() {
                commands.entity(entity).insert(IsPowered);
            } else {
                commands.entity(entity).remove::<IsPowered>();
            }
        });
    }
}

/// Render all the circles of power as transparent yellow circles
fn debug_power_map_ui(
    mut gizmos: Gizmos,
    q_supply: Query<(&SupplyRadius, &Transform)>,
    q_powered_buildings: Query<&Transform, With<IsPowered>>,
    q_unpowered_buildings: Query<(&Building, &Transform), Without<IsPowered>>,
) {
    q_supply.iter().for_each(|(radius, transform)| {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        gizmos.circle_2d(pos, radius.0, Color::YELLOW).segments(32);
    });
    q_powered_buildings.iter().for_each(|transform| {
        let pos = Vec2::new(transform.translation.x, transform.translation.y + 26.0);
        gizmos.circle_2d(pos, 7.0, Color::GREEN).segments(16);
    });
    q_unpowered_buildings.iter().for_each(|(_, transform)| {
        let pos = Vec2::new(transform.translation.x, transform.translation.y + 26.0);
        gizmos.circle_2d(pos, 7.0, Color::RED).segments(16);
    });
}
