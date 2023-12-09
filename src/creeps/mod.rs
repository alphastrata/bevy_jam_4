use std::ops::ControlFlow;

use crate::prelude::*;
use bevy::{asset::processor::ProcessorTransactionLog, ecs::bundle, prelude::*};
use rand::Rng;

pub struct CreepPlugin;
impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.insert_resource(CreepCount(0));

        app.add_event::<SpawnCreep>();

        app.add_systems(Update, initial_creep_spawn);

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
    (0..1000)
        .into_iter()
        .for_each(|_| spawn_creep(&mut commands, &asset_server));
}

fn spawn_creep(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    //TODO: random xy spawn at distance from center of map...
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
            Experience(100),
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

fn attack_tower() {
    todo!()
}
