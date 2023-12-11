//! A building that drains **all** trees in close proximity.
//!
//! **LOGIC:**
//!
//! On [SpawnCreep] we recalculate the trees that a particular [DrainTower] can harvest from.
//! This is because we don't want to do a N^2 every frame.
//!
//! Every frame we try and drain a tree

use crate::{
    creeps::{CreepDie, SpawnCreep},
    game::power::{AddBuilding, IsPowered, RequiresPower},
    AnimationIndices, AnimationTimer, AppState, Health, Teardown, Tree,
};
use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    tasks::IoTaskPool,
};

use super::{twr_custom_mats::TowerRadiusMaterial, Building, BuildingDefinition};

/// Drain damage applied to trees per tick of [GlobalDrainTick]
const DRAIN_DPT: u32 = 2;
/// Every *this* many seconds trees get drained
const DRAIN_TICK_RATE: f32 = 0.15;

#[derive(Component)]
pub struct DrainRadius(f32);

#[derive(Resource)]
struct GlobalDrainTick(Timer);

#[derive(Component, Default)]
pub struct DrainTower {
    trees_in_proximity: Vec<Entity>,
}

impl BuildingDefinition for DrainTower {
    const SPRITE_PATH: &'static str = "textures/sucky-uppy.png";
    const BASE_HEALTH: u32 = 100;
    const COST: u32 = 300;
    const BUILD_TIME: u32 = 5;
    const NAME: &'static str = "Drain Tower";
    const DESCRIPTION: &'static str = "The Drain Tower slowly drains the health of
        closeby towers. Upgrading it increases it's active radius.";

    fn add_extra_components(commands: &mut Commands, ent_id: Entity) {
        commands
            .entity(ent_id)
            .insert((RequiresPower, DrainRadius(400.0), DrainTower::default()));
    }
}

impl DrainTower {
    /// Manually implement the spawning since it uses SpriteSheetBundle and animatin components
    /// compared to the simple [MinimalBuilding] Bundle
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
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 64.0), 18, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let succ_anim = AnimationIndices { first: 1, last: 17 };

        let radius_display = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(400.).into()).into(),
                material: materials.add(TowerRadiusMaterial {
                    color: Color::PURPLE,
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                ..default()
            })
            .id();

        let ent_id = commands
            .spawn((
                Building,
                Teardown,
                Health(Self::BASE_HEALTH),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(succ_anim.first),
                    transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.01)),
                    ..default()
                },
                succ_anim,
                AnimationTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
            ))
            .add_child(radius_display)
            .id();

        Self::add_extra_components(commands, ent_id);
        ent_id
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
            (
                animate_sprite,
                calculate_drainees,
                drain_closeby_trees,
                debug_drain_radii,
            )
                .run_if(in_state(AppState::Gameplay)),
        );
    }
}

/// We don't want to calculate the trees in range of a tower every single frame or tick
/// so this system instead calculates them every time new creeps are spawned or a new
/// tower is built.
fn calculate_drainees(
    tower_spawned: EventReader<AddBuilding>,
    creep_spawned: EventReader<SpawnCreep>,
    mut q_towers: Query<(&mut DrainTower, &Transform, &DrainRadius)>,
    creep_died: EventReader<CreepDie>,
    q_trees: Query<(Entity, &Transform), With<Tree>>,
) {
    let mut total_trees = 0;
    if !(creep_spawned.is_empty() && tower_spawned.is_empty() && creep_died.is_empty()) {
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
    q_towers: Query<&DrainTower, With<IsPowered>>,
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
                    debug!("hp rem: {}", hp.0);
                }
            });
        });
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        (With<DrainTower>, With<IsPowered>),
    >,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn debug_drain_radii(_gizmos: Gizmos, _q_towers: Query<(&DrainRadius, &Transform)>) {
    // q_towers.iter().for_each(|(radius, transform)| {
    //     let pos = Vec2::new(transform.translation.x, transform.translation.y);
    //     gizmos.circle_2d(pos, radius.0, Color::CYAN).segments(32);
    // });
}
