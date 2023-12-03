//! Different tower types and their functionality

use bevy::prelude::*;

/// Marker component for towers
#[derive(Component)]
pub struct Tower;

/// Blueprint for a generic tower entity
#[derive(Bundle)]
pub struct TowerBundle {
    marker: Tower,
    health: Health,
    exp: Experience,
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Health(u32);

#[derive(Component)]
pub struct Experience(u32);

// TODO: generic definition of a "Tower"?
pub trait TowerDefinition {
    // fn spawn_tower()
}

pub fn spawn_fire_tower(mut commands: Commands, pos: Vec2) {
    commands.spawn(TowerBundle {
        marker: Tower,
        health: Health(100),
        exp: Experience(0),
        sprite: SpriteBundle {
            // TODO: replace with a tower image asset
            sprite: Sprite {
                color: Color::rgb(0.9, 0.1, 0.1),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            ..default()
        },
    });
}
