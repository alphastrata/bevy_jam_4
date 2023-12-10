//! Generic systems that update HP bar UI for any entity with a [Health] component.

use bevy::{prelude::*, sprite::Anchor};

use crate::{AppState, Health};

const HP_BAR_THICCNESS: f32 = 8.0;
const HP_BAR_WIDTHNESS: f32 = 80.0;

#[derive(Component)]
pub struct HpBarUISettings {
    /// max hp of the entity
    pub max: u32,
    /// offset the hp bar from the top translation of the parent component
    pub offset: Option<Vec2>,
}

/// (Red bg sprite, green happy sprite)
#[derive(Component)]
pub struct HpBarUI((Entity, Entity));

#[derive(Component)]
pub struct HpBarRed;
#[derive(Component)]
pub struct HpBarGreen;

pub struct HealthBarUIPlugin;
impl Plugin for HealthBarUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_new_hp_bar, update_bars).run_if(in_state(AppState::Gameplay)),
        );
    }
}

fn spawn_new_hp_bar(
    mut commands: Commands,
    new_query: Query<(Entity, &Health, &HpBarUISettings), Added<HpBarUISettings>>,
) {
    new_query.iter().for_each(|(ent, _health, settings)| {
        let offset = settings.offset.unwrap_or(Vec2::new(0.0, -64.0));
        let hp_bar_red_bg = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(HP_BAR_WIDTHNESS, HP_BAR_THICCNESS)),
                        anchor: Anchor::BottomCenter,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(offset.x, offset.y, 0.29)),
                    ..default()
                },
                HpBarRed,
            ))
            .id();

        let hp_bar_green_rem = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(HP_BAR_WIDTHNESS, HP_BAR_THICCNESS)),
                        anchor: Anchor::BottomCenter,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(offset.x, offset.y, 0.3)),
                    ..default()
                },
                HpBarGreen,
            ))
            .id();

        if let Some(mut ent_cmds) = commands.get_entity(ent) {
            ent_cmds
                .insert(HpBarUI((hp_bar_red_bg, hp_bar_green_rem)))
                .add_child(hp_bar_red_bg)
                .add_child(hp_bar_green_rem);
        }
    });
}

/// Runs when any entity with a [Health] component has changed and updates the
/// Health bar UI
fn update_bars(
    changed_query: Query<(&Health, &HpBarUISettings, &HpBarUI), Changed<Health>>,
    mut q_green: Query<(&mut Sprite, &mut Transform), With<HpBarGreen>>,
) {
    changed_query.iter().for_each(|(health, settings, ui)| {
        let Ok((mut green_sprite, mut green_tf)) = q_green.get_mut(ui.0 .1) else {
            return;
        };

        let percent = (health.0 as f32 / settings.max as f32).clamp(0.0, 1.0);
        trace!("Entity hp reduced to {} percent", percent);
        let x_offset = (1.0 - percent) * HP_BAR_WIDTHNESS;
        green_sprite.custom_size = Some(Vec2::new(HP_BAR_WIDTHNESS * percent, HP_BAR_THICCNESS));
        let offset = settings.offset.unwrap_or(Vec2::new(0.0, -64.0));
        *green_tf = Transform::from_translation(Vec3::new(-x_offset / 2.0, offset.y, 0.3));
    });
}
