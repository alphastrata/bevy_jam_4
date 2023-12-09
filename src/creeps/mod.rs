use std::ops::ControlFlow;

use crate::{buildings::Building, prelude::*, Range};
use bevy::{asset::processor::ProcessorTransactionLog, ecs::bundle, prelude::*};
use rand::Rng;

pub struct CreepPlugin;
impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCreep>();

        app.add_systems(Startup, initial_creep_spawn)
            .add_systems(Update, spawn_on_trigger);

        #[cfg(debug_assertions)]
        app.insert_resource(CreepCount(0));
        #[cfg(debug_assertions)]
        app.add_systems(Update, (dbg_send_spawn_creep_on_enter, dbg_count_creeps));
    }
}

#[derive(Component)]
pub struct Creep;

#[derive(Event)]
pub struct SpawnCreep;

#[cfg(debug_assertions)]
#[derive(Resource)]
pub struct CreepCount(pub usize);

#[cfg(debug_assertions)]
fn dbg_count_creeps(q: Query<&Transform, With<Creep>>, mut count: ResMut<CreepCount>) {
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
    (0..1000).for_each(|_| spawn_creep(&mut commands, &asset_server));
}

/// Helper: for the Systems: [initial_creep_spawn, spawn_on_trigger]
fn spawn_creep(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let eps = 8192.0;
    let x = rng.gen_range(-eps..=eps);
    let y = rng.gen_range(-eps..=eps);

    let current = Vec3::new(x, y, 0.0);
    let target = Vec3::ZERO;
    let direction = target - current;

    if direction.length() > 240.0 {
        // Approximately the circle of the 'core'
        //TODO: Keep a creep count, resource for UI?
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.10),
                texture: asset_server.load("textures/creep.png"),
                //
                ..default()
            },
            Creep,
            //TODO: creep stats should probs be set by .csv or something?
            AttackSpeed(10),
            Health(100),
            Range(300),
            CorpoPoints(rng.gen_range(1.0..50.0) as u32),
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
fn attack_towers(
    q_building: Query<(&Transform, &AttackSpeed, &Range, &Health), With<Building>>,
    q_creep: Query<(&Transform, &AttackSpeed, &Range, &Health), With<Creep>>,
    _time: Res<Time>, // Replace with Josh's ticker as needed
) {
    q_building
        .iter()
        .for_each(|(building_transform, _, building_range, building_health)| {
            if building_health.0 > 0 {
                q_creep
                    .iter()
                    .for_each(|(creep_transform, _, _, creep_health)| {
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
fn cleanup_dead(mut commands: Commands, q: Query<(Entity, &Health)>) {
    q.iter()
        .filter(|(_entity, health)| health.0 <= 0)
        .for_each(|(entity, _health)| {
            commands.entity(entity).despawn();
        });
}
