//! Creeps are the enemy! They are also known as "Tree"s.
use std::{any, ops::ControlFlow, time::Duration};

use crate::{
    buildings::Building,
    game::{
        hp_bars::HpBarUISettings,
        resources::{Harvest, ResourceType},
    },
    global_systems::eargasm::{AudioComponent, AudioRequest, Money},
    prelude::*,
    Range, Teardown,
};
use bevy::{
    asset::processor::ProcessorTransactionLog, ecs::bundle, prelude::*, sprite::Mesh2dHandle,
    time::Stopwatch,
};
use bevy_hanabi::prelude::*;
use rand::Rng;

/// Handles the setup, spawning, despawning, attacking of our 'creeps'.
pub struct CreepPlugin;
impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCreep>();
        app.add_event::<CreepDie>();

        app.add_systems(Startup, (initial_creep_spawn, creep_spawning_timer))
            .add_systems(
                Update,
                (
                    cleanup_dead_creeps,
                    periodically_spawn_creep,
                    periodically_add_particle,
                ),
            );
    }
}

#[derive(Event)]
pub struct SpawnCreep;

#[derive(Resource)]
struct CreepSpawnTimer {
    timer: Stopwatch,
}

#[derive(Resource)]
struct ParticleTimer {
    timer: Stopwatch,
}

#[derive(Resource)]
struct TreeCreepAtlas(Handle<TextureAtlas>);

/// System: Setup
fn initial_creep_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    (0..10_000).for_each(|_| spawn_creep(&mut commands, &asset_server, &mut texture_atlases));
}

fn creep_spawning_timer(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    commands.insert_resource(CreepSpawnTimer {
        timer: Stopwatch::new(),
    });
    commands.insert_resource(ParticleTimer {
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    creep_timer.timer.tick(time.delta());
    if creep_timer.timer.elapsed_secs() > 2.3 {
        (0..80).for_each(|_| spawn_creep(&mut commands, &asset_server, &mut texture_atlases));
        creep_timer.timer.reset()
    }
}

fn periodically_add_particle(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_timer: ResMut<ParticleTimer>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    particle_timer.timer.tick(time.delta());
    if particle_timer.timer.elapsed_secs() > 1.0 {
        // Create a color gradient for the particles
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.5, 0.5, 1.0, 1.0));
        gradient.add_key(1.0, Vec4::new(0.5, 0.5, 1.0, 0.0));

        let writer = ExprWriter::new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let lifetime = writer.lit(5.).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(0.05).expr(),
            dimension: ShapeDimension::Surface,
        };

        let init_vel = SetVelocityCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            speed: writer.lit(0.1).expr(),
        };

        let spawner = Spawner::rate(100.0.into());
        let effect = effects.add(
            EffectAsset::new(4096, spawner, writer.finish())
                .with_name("2d")
                .init(init_pos)
                .init(init_vel)
                .init(init_age)
                .init(init_lifetime)
                .render(SizeOverLifetimeModifier {
                    gradient: Gradient::constant(Vec2::splat(0.02)),
                    screen_space_size: false,
                })
                .render(ColorOverLifetimeModifier { gradient }),
        );

        commands
            .spawn(ParticleEffectBundle {
                effect: ParticleEffect::new(effect).with_z_layer_2d(Some(1.0)),
                ..default()
            })
            .insert(Name::new("effect:2d"));

        info!("Added Particle");
        particle_timer.timer.reset();
    }
}

fn spawn_creep(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    const MAP_LIMIT: f32 = 8192.0;

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-MAP_LIMIT..=MAP_LIMIT);
    let y = rng.gen_range(-MAP_LIMIT..=MAP_LIMIT);

    let current = Vec3::new(x, y, 0.0);
    let target = Vec3::ZERO;
    let direction = target - current;

    let texture_handle = asset_server.load("textures/trees.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    if direction.length() > 240.0 {
        let atlas_handle = texture_atlas_handle.clone();
        let sprite_index = rng.gen_range(0..9);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: atlas_handle,
                sprite: TextureAtlasSprite::new(sprite_index),
                transform: Transform::from_xyz(x, y, 0.10),
                ..default()
            },
            Teardown,
            Tree,
            AttackSpeed(10),
            Health(100),
            HpBarUISettings {
                max: 100,
                offset: Some(Vec2::new(0.0, -32.0)),
            },
            Range(300),
            CorpoPoints(rng.gen_range(1.0..50.0) as u32),
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

#[derive(Event)]
pub struct CreepDie;

/// System: Update, remove anything with Health 0.
fn cleanup_dead_creeps(
    mut commands: Commands,
    mut harvest: EventWriter<Harvest>,
    q: Query<(Entity, &Health, &CorpoPoints), With<Tree>>,
    mut audio_mngr: EventWriter<AudioRequest>,
    mut creep_die: EventWriter<CreepDie>,
) {
    q.iter()
        .filter(|(_entity, health, _)| health.0 == 0)
        .for_each(|(entity, _health, corpo_pts)| {
            harvest.send(Harvest(ResourceType::CorporationPoints, corpo_pts.0));
            audio_mngr.send(AudioRequest {
                component: AudioComponent::Money(Money),
            });
            creep_die.send(CreepDie);

            commands.entity(entity).despawn_recursive();
        });
}
