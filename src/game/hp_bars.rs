//! Generic systems that update HP bar UI for any entity with a [Health] component.

use bevy::{prelude::*, sprite::Anchor};

use crate::{AppState, Health};

const HP_BAR_THICCNESS: f32 = 10.0;
const HP_BAR_WIDTHNESS: f32 = 100.0;

#[derive(Component)]
pub struct HpBarUISettings {
    /// max hp of the entity
    pub max: u32,
    /// offset the hp bar from the top translation of the parent component
    pub offset: Option<Vec2>,
}

#[derive(Component)]
pub struct HpBarUI(Entity);

pub struct HealthBarUIPlugin;
impl Plugin for HealthBarUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_new_hp_bar).run_if(in_state(AppState::Gameplay)),
        );
    }
}

fn spawn_new_hp_bar(
    mut commands: Commands,
    new_query: Query<(Entity, &Health, &HpBarUISettings), Added<HpBarUISettings>>,
) {
    new_query.iter().for_each(|(ent, _health, settings)| {
        info!("New HP bar being spawned");
        let offset = settings.offset.unwrap_or(Vec2::new(0.0, -64.0));
        let hp_bar_red_bg = commands
            .spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(HP_BAR_WIDTHNESS, HP_BAR_THICCNESS)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(offset.x, offset.y, 0.29)),
                ..default()
            },))
            .id();

        let hp_bar_green_rem = commands
            .spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(HP_BAR_WIDTHNESS, HP_BAR_THICCNESS)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(offset.x, offset.y, 0.3)),
                ..default()
            },))
            .id();

        if let Some(mut ent_cmds) = commands.get_entity(ent) {
            ent_cmds
                .add_child(hp_bar_red_bg)
                .add_child(hp_bar_green_rem);
        }
    });

    // commands.spawn
}

/// Runs when any entity with a [Health] component has changed and updates the
/// Health bar UI
fn update_bars(_changed_query: Query<(&Health, &HpBarUI), Changed<Health>>) {}
