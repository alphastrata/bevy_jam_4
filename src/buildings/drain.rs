//! A building that drains **all** trees in close proximity.
//!
//! **LOGIC:**
//!
//! On [SpawnCreep] we recalculate the trees that a particular [DrainTower] can harvest from.
//! This is because we don't want to do a N^2 every frame.
//!
//! Every frame we try and drain a tree

use crate::{creeps::SpawnCreep, game::power::AddBuilding, AppState, Health, Tree};
use bevy::prelude::*;

use super::BuildingDefinition;

/// Drain damage applied to trees per tick of [GlobalDrainTick]
const DRAIN_DPT: u32 = 20;
/// Every *this* many seconds trees get drained
const DRAIN_TICK_RATE: f32 = 1.5;

#[derive(Component)]
pub struct DrainRadius(f32);

#[derive(Resource)]
struct GlobalDrainTick(Timer);

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
        commands
            .entity(ent_id)
            .insert((DrainRadius(150.0), DrainTower::default()));
    }
}

pub struct DrainTowerPlugin;
impl Plugin for DrainTowerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalDrainTick(Timer::from_seconds(
            DRAIN_TICK_RATE,
            TimerMode::Repeating,
        )))
        .add_event::<AddBuilding>()
        .add_event::<SpawnCreep>()
        .add_systems(
            Update,
            (calculate_drainees, drain_closeby_trees).run_if(in_state(AppState::Gameplay)),
        );
    }
}

/// We don't want to calculate the trees in range of a tower every single frame or tick
/// so this system instead calculates them every time new creeps are spawned.
fn calculate_drainees(
    mut tower_spawned: EventReader<AddBuilding>,
    mut creep_spawned: EventReader<SpawnCreep>,
    mut q_towers: Query<(&mut DrainTower, &Transform, &DrainRadius)>,
    q_trees: Query<(Entity, &Transform), With<Tree>>,
) {
    let mut total_trees = 0;
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
                total_trees += close_trees.len();

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
                total_trees += close_trees.len();

                dt.trees_in_proximity = close_trees;
            });
    });
    if !(creep_spawned.is_empty() && tower_spawned.is_empty()) {
        trace!("Recalculated trees within range of Drain Towers (tower build)\n {} trees are being sucked", total_trees);
    }
}

/// Each [DrainTower] slowly drains the health of every close tree
///
/// NOTE: we don't care about what happens after a tree dies (hp -> zero) here, that should be
/// handled in other systems.
fn drain_closeby_trees(
    mut timer: ResMut<GlobalDrainTick>,
    mut q_trees: Query<(Entity, &mut Health), With<Tree>>,
    q_towers: Query<&DrainTower>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        trace!(
            "Drain Tick: draining from {} Drain Towers",
            q_towers.iter().len()
        );
        // for each tower check all the trees in proximity and deduct hp from them.
        q_towers.iter().for_each(|tower: &DrainTower| {
            tower.trees_in_proximity.iter().for_each(|ent| {
                if let Ok((_tree, mut hp)) = q_trees.get_mut(*ent) {
                    hp.deduct(DRAIN_DPT);
                }
            });
        });
    }
}

fn debug_drain_radii() {
    // TODO
}
