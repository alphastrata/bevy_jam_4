//! Creeps are the enemy! They are also known as "Tree"s.

use std::{ops::ControlFlow, time::Duration};

use crate::{
    buildings::Building,
    game::{
        hp_bars::HpBarUISettings,
        resources::{Harvest, ResourceType},
    },
    global_systems::eargasm::{AudioRequest, Money},
    prelude::*,
    AppState, Range,
};
use bevy::{asset::processor::ProcessorTransactionLog, ecs::bundle, prelude::*, time::Stopwatch};
use rand::Rng;

/// Handles the setup, spawning, despawning, attacking of our 'creeps'.
pub struct CreepPlugin;
impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCreep>();

        app.add_systems(
            OnEnter(AppState::Gameplay),
            (initial_creep_spawn, creep_spawning_timer),
        )
        .add_systems(OnExit(AppState::Gameplay), teardown)
        .add_systems(
            Update,
            (periodically_spawn_creep, cleanup_dead_creeps).run_if(in_state(AppState::Gameplay)),
        );
    }
}

#[derive(Event)]
pub struct SpawnCreep;

#[derive(Resource)]
struct CreepSpawnTimer {
    timer: Stopwatch,
}

/// System: Setup
fn initial_creep_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    (0..10_000).for_each(|_| spawn_creep(&mut commands, &asset_server));
}

fn creep_spawning_timer(mut commands: Commands) {
    commands.insert_resource(CreepSpawnTimer {
        timer: Stopwatch::new(),
    });
}
/// System: Update
/// Does what it says on the can...
fn periodically_spawn_creep(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut creep_timer: ResMut<CreepSpawnTimer>,
) {
    creep_timer.timer.tick(time.delta());
    if creep_timer.timer.elapsed_secs() > 30.0 {
        (0..500).for_each(|_| spawn_creep(&mut commands, &asset_server));
        creep_timer.timer.reset()
    }
}

pub fn teardown(mut commands: Commands, q: Query<(Entity, &Health, &CorpoPoints), With<Tree>>) {
    q.iter().for_each(|(entity, _health, _corpo_pts)| {
        commands.entity(entity).despawn();
    });
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
    mut audio_mngr: EventWriter<AudioRequest>,
) {
    q.iter()
        .filter(|(_entity, health, _)| health.0 == 0)
        .for_each(|(entity, _health, corpo_pts)| {
            harvest.send(Harvest(ResourceType::CorporationPoints, corpo_pts.0));
            audio_mngr.send(AudioRequest {
                component: crate::global_systems::eargasm::AudioComponent::Money(Money),
            });
            commands.entity(entity).despawn_recursive();
        });
}
