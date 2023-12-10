//! Creeps are the enemy! They are also known as "Tree"s.

use std::ops::ControlFlow;

use crate::{
    buildings::Building,
    game::{
        hp_bars::HpBarUISettings,
        resources::{Harvest, ResourceType},
    },
    prelude::*,
    Range, Teardown,
};
use bevy::{asset::processor::ProcessorTransactionLog, ecs::bundle, prelude::*};
use rand::Rng;

/// Handles the setup, spawning, despawning, attacking of our 'creeps'.
pub struct CreepPlugin;
impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCreep>();

        app.add_systems(Startup, initial_creep_spawn)
            .add_systems(Update, (spawn_on_trigger, cleanup_dead_creeps));

        #[cfg(debug_assertions)]
        app.insert_resource(CreepCount(1000)); //TODO: maybe we ensure a certain 'minimum' ammount of creeps at any one time?
                                               // #[cfg(debug_assertions)]
                                               //app.add_systems(Update, (dbg_send_spawn_creep_on_enter, dbg_count_creeps));
    }
}

#[derive(Event)]
pub struct SpawnCreep;

#[cfg(debug_assertions)]
#[derive(Resource)]
pub struct CreepCount(pub usize);

#[cfg(debug_assertions)]
fn dbg_count_creeps(q: Query<&Transform, With<Tree>>, mut count: ResMut<CreepCount>) {
    use crate::Tree;

    (*count) = CreepCount(q.iter().count());
}

#[cfg(debug_assertions)]
fn dbg_send_spawn_creep_on_enter(mut spawner: EventWriter<SpawnCreep>, kb: Res<Input<KeyCode>>) {
    if kb.just_released(KeyCode::Return) {
        spawner.send(SpawnCreep);
    }
}

/// System: Setup
fn initial_creep_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    (0..10000).for_each(|_| spawn_creep(&mut commands, &asset_server));
}

/// Helper: for the Systems: [initial_creep_spawn, spawn_on_trigger]
fn spawn_creep(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    const MAP_LIMIT: f32 = 8192.0;

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-MAP_LIMIT..=MAP_LIMIT);
    let y = rng.gen_range(-MAP_LIMIT..=MAP_LIMIT);

    let current = Vec3::new(x, y, 0.0);
    let target = Vec3::ZERO;
    let direction = target - current;

    if direction.length() > 240.0 {
        //TODO: Random selection of sprite from upcomming options.
        //TODO: remove placeholder creep.png
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.10),
                texture: asset_server.load("textures/creep.png"),
                //
                ..default()
            },
            Teardown,
            Tree,
            AttackSpeed(10), //TODO: multiply out by the tick?, QUESTION: relative to the sprite we load?
            Health(100),     //QUESTION: relative to the sprite we load?
            HpBarUISettings {
                max: 100,
                offset: Some(Vec2::new(0.0, -32.0)),
            },
            Range(300), //QUESTION: relative to the sprite we load?
            CorpoPoints(rng.gen_range(1.0..50.0) as u32), //QUESTION: relative to the sprite we load?
        ));
    }
}

/// System: Update, listens for the Event<SpawnCreep>, when it does we'll it'll spawn a creep at a random location
fn spawn_on_trigger(
    mut spawner: EventReader<SpawnCreep>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawner
        .read()
        .for_each(|_| spawn_creep(&mut commands, &asset_server));
}

/// System: Update 'attack' the closest tower.
//TODO: generify and make the attack<T> where T could be either the creep or building
fn attack_towers(
    q_building: Query<(&Transform, &AttackSpeed, &Range, &Health), With<Building>>,
    q_creep: Query<(&Transform, &AttackSpeed, &Range, &Health), With<Tree>>,
    _time: Res<Time>, // Replace with Josh's ticker as needed
) {
    q_building
        .iter()
        .for_each(|(building_transform, _, building_range, building_health)| {
            // Can the building attack?
            if building_health.0 > 0 {
                q_creep
                    .iter()
                    .for_each(|(creep_transform, _, _, creep_health)| {
                        // Is it worth it?
                        if creep_health.0 > 0 {
                            let distance = building_transform
                                .translation
                                .distance(creep_transform.translation);

                            if distance as u32 <= building_range.0 {
                                // Logic to 'attack' - marking for attack or similar
                                // This could be an event trigger or direct action
                                // TODO trigger animation?
                            }
                        }
                    });
            }
        });
}

/// System: Update, remove anything with Health 0.
fn cleanup_dead_creeps(
    mut commands: Commands,
    mut harvest: EventWriter<Harvest>,
    q: Query<(Entity, &Health, &CorpoPoints), With<Tree>>,
) {
    q.iter()
        .filter(|(_entity, health, _)| health.0 == 0)
        .for_each(|(entity, _health, corpo_pts)| {
            harvest.send(Harvest(ResourceType::CorporationPoints, corpo_pts.0));
            commands.entity(entity).despawn_recursive();
        });
}
