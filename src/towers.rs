//! Different tower types and their functionality
use bevy::prelude::*;

use bevy_mod_picking::prelude::*;

/// Representing the types of tower we have
#[derive(Component, Debug, PartialEq, Eq)]
pub enum TowerType {
    /// Combats wind
    Fan,
    /// tbd
    Shield,
    /// tbd
    Radar,
    /// tbd
    Doppler,
    /// tbd
    Distribution,
}

/// Component:
/// Housing everything and anything that a Tower will USE in game..
#[derive(Component, Debug, PartialEq, Eq)]
pub struct Tower {
    tower_type: TowerType,
}
impl Default for Tower {
    fn default() -> Self {
        Self {
            tower_type: TowerType::Distribution,
        }
    }
}

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
    commands.spawn((TowerBundle {
        marker: Tower { ..default() },
        health: Health(100),
        exp: Experience(0),
        sprite: SpriteBundle {
            // TODO: replace with a tower image asset
            sprite: Sprite {
                color: Color::rgb(0.9, 0.1, 0.1),
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            ..default()
        },

        
    }
            On::<Pointer<Over>>::send_event::<Greeting>())
    
    );
}
