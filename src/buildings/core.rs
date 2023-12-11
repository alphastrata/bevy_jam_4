use crate::{
    game::power::{IsPowered, SupplyRadius},
    AnimationIndices, AnimationTimer, AppState, Health, Teardown, BUILDING_Z, SHADER_Z,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::{twr_custom_mats::TowerRadiusMaterial, Building, BuildingDefinition};

/// Core building that the player starts with
#[derive(Component, Default)]
pub struct TheCore;
impl BuildingDefinition for TheCore {
    const SPRITE_PATH: &'static str = "textures/core-spritesheet.png";
    const BASE_HEALTH: u32 = 1000;
    const COST: u32 = 0; // Core is free since you start with it
    const BUILD_TIME: u32 = 0;
    const NAME: &'static str = "Core";
    const DESCRIPTION: &'static str = "";

    fn add_extra_components(commands: &mut Commands, ent_id: Entity) {
        commands
            .entity(ent_id)
            .insert((IsPowered, SupplyRadius(550.0), TheCore));
    }
}

impl TheCore {
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
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 96.0), 8, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let core_anim = AnimationIndices { first: 0, last: 7 };

        let radius_display = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(550.).into()).into(),
                material: materials.add(TowerRadiusMaterial {
                    color: Color::PURPLE,
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, SHADER_Z)),
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
                    sprite: TextureAtlasSprite::new(core_anim.first),
                    transform: Transform::from_translation(Vec3::new(pos.x, pos.y, BUILDING_Z)),
                    ..default()
                },
                core_anim,
                AnimationTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
            ))
            .add_child(radius_display)
            .id();

        Self::add_extra_components(commands, ent_id);
        ent_id
    }
}

pub struct TheCorePlugin;
impl Plugin for TheCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_sprite,).run_if(in_state(AppState::Gameplay)),
        );
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
        With<TheCore>,
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
