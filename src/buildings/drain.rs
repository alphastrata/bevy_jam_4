//! A building that drains **all** trees in close proximity.

use crate::{creeps::SpawnCreep, game::power::AddBuilding, AppState, Health, Tree};
use bevy::prelude::*;

use super::BuildingDefinition;

#[derive(Component)]
pub struct DrainRadius(f32);

#[derive(Default, Component)]
pub struct DrainTower {
    trees_in_proximity: Vec<Entity>,
}
impl BuildingDefinition for DrainTower {
    const SPRITE_PATH: &'static str = "textures/tower.png";
    const BASE_HEALTH: u32 = 100;
    const COST: u32 = 20;
    const BUILD_TIME: u32 = 5;
    const NAME: &'static str = "Drain Tower";
    const DESCRIPTION: &'static str = "The Drain Tower slowly drains the health of
        closeby towers. Upgrading it increases it's active radius.";

    fn add_extra_components(commands: &mut Commands, ent_id: Entity) {
        commands.entity(ent_id).insert(DrainRadius(150.0));
    }
}

pub struct DrainTowerPlugin;
impl Plugin for DrainTowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (calculate_drainees, drain_closeby_trees).run_if(in_state(AppState::Gameplay)),
        );
    }
}

/// We don't want to calculate the trees in range of a tower every single frame or tick
/// so instead calculate them when new creeps are spawned.
fn calculate_drainees(
    mut tower_spawned: EventReader<AddBuilding>,
    mut creep_spawned: EventReader<SpawnCreep>,
    mut q_towers: Query<(&mut DrainTower, &Transform, &DrainRadius)>,
    q_trees: Query<(Entity, &Transform), With<Tree>>,
) {
    creep_spawned.read().for_each(|_| {
        q_towers
            .iter_mut()
            .for_each(|(mut dt, tower_tf, tower_radius)| {
                let close_trees: Vec<_> = q_trees
                    .iter()
                    .filter(|(_, tree_tf): &(Entity, &Transform)| {
                        tree_tf.translation.distance(tower_tf.translation) < tower_radius.0
                    })
                    .map(|(tree, _)| tree)
                    .collect();

                dt.trees_in_proximity = close_trees;
            });
    });
    // Yes, I duplicated it. curse you, this is a game jam, let me be pasta-y!
    tower_spawned.read().for_each(|_| {
        q_towers
            .iter_mut()
            .for_each(|(mut dt, tower_tf, tower_radius)| {
                let close_trees: Vec<_> = q_trees
                    .iter()
                    .filter(|(_, tree_tf): &(Entity, &Transform)| {
                        tree_tf.translation.distance(tower_tf.translation) < tower_radius.0
                    })
                    .map(|(tree, _)| tree)
                    .collect();

                dt.trees_in_proximity = close_trees;
            });
    });
}

/// Each [DrainTower] slowly drains the health of every close tree
///
/// NOTE: we don't care about what happens after a tree dies (hp -> zero) here, that should be
/// handled in other systems.
fn drain_closeby_trees(
    q_towers: Query<&DrainTower>,
    mut q_trees: Query<(Entity, &mut Health), With<Tree>>,
) {
    q_towers.iter().for_each(|tower: &DrainTower| {
        tower.trees_in_proximity.iter().for_each(|ent| {
            if let Ok((_tree, mut hp)) = q_trees.get_mut(*ent) {
                hp.deduct(5);
            }
        });
    });
}
