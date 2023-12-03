//! Different tower types and their functionality
use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::prelude::*;

/// Representing the types of tower we have
#[derive(Clone, Hash, Component, Debug, PartialEq, Eq)]
pub enum TowerType {
    /// Combats wind
    Fan,

    /// tbd
    Shield,

    /// tbd
    Radar,

    /// tbd
    Doppler,

    /// Pylons, you must construct additional...
    Distribution,

    ///
    Roboport,

    ///
    LoggingCentre,
}

/// Blueprint for a generic tower entity
#[derive(Bundle)]
pub struct TowerBundle {
    marker: TowerType,
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
    commands.spawn((
        TowerBundle {
            marker: TowerType::Distribution,
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
        },
        PickableBundle::default(), // <- Makes the mesh pickable.
        HIGHLIGHT_TINT,            // Override the global highlighting settings for this mesh
    ));
}

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.2, -0.2, 0.4, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, -0.3, 0.5, 0.0),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, 0.2, -0.3, 0.0),
        ..matl.to_owned()
    })),
};
