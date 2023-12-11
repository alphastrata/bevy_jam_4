use std::fmt::format;

use bevy::prelude::*;

use crate::{
    global_systems::{
        eargasm::{AudioRequest, Track2},
        ui_util::GameFont,
    },
    AppState,
};

use super::hud::PIXEL;

const DEFAULT_PURSE_SIZE: u32 = 1000;

pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_event::<ExpendResource>()
            .add_event::<Harvest>()
            .add_systems(OnEnter(AppState::Gameplay), setup_debug_ui)
            .add_systems(OnExit(AppState::Gameplay), (teardown_debug_ui, reset_money))
            .add_systems(
                Update,
                (update_debug_ui, add_harvest_to_inventory, expend_resource),
            );
    }
}

/// What the player currently has in the BANK
#[derive(Resource, Clone)]
pub struct Inventory {
    pub money: u32,
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            money: DEFAULT_PURSE_SIZE,
        }
    }
}

pub enum ResourceType {
    CorporationPoints,
}

#[derive(Event)]
pub struct ExpendResource(pub ResourceType, pub u32);

/// This event should be fired when a resource was harvested
/// (resource, money_earned)
#[derive(Event)]
pub struct Harvest(pub ResourceType, pub u32);

/// System:
/// Changes the music of the game based on how much money you have.
fn game_stage_music_choice(inventory: Res<Inventory>, mut audio_mngr: EventWriter<AudioRequest>) {
    if inventory.money > 1500 {
        audio_mngr.send(AudioRequest {
            component: crate::global_systems::eargasm::AudioComponent::Track2(Track2),
        })
    }
}

/// System that adds all harvested resources to the players inventory
fn add_harvest_to_inventory(mut inventory: ResMut<Inventory>, mut harvests: EventReader<Harvest>) {
    *inventory = harvests.read().fold(inventory.clone(), |mut inv, harvest| {
        match harvest.0 {
            ResourceType::CorporationPoints => inv.money += harvest.1,
        };
        inv
    });
}

fn expend_resource(mut inventory: ResMut<Inventory>, mut expent: EventReader<ExpendResource>) {
    *inventory = expent.read().fold(inventory.clone(), |mut inv, harvest| {
        match harvest.0 {
            ResourceType::CorporationPoints => inv.money = inv.money.saturating_sub(harvest.1),
        };
        inv
    });
}

/// Marker component for despawning inventory UI later
#[derive(Component)]
struct InventoryDebugUI;

#[derive(Component)]
struct WoodNumber;

/// Ugly UI for temporarily showing inventory. Will be beautified later!
fn setup_debug_ui(mut commands: Commands, font: Res<GameFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                z_index: ZIndex::Global(i32::MAX - 1),
                ..default()
            },
            InventoryDebugUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Wood: 0",
                    TextStyle {
                        font_size: 42.0,
                        font: font.0.clone(),
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    width: Val::Px(102.0 * PIXEL),
                    margin: UiRect::all(Val::Px(15.)),
                    ..default()
                }),
                WoodNumber,
            ));
        });
}

fn update_debug_ui(mut q_text: Query<&mut Text, With<WoodNumber>>, inventory: Res<Inventory>) {
    for mut text in &mut q_text {
        text.sections[0].value = format!("Corpo Points: {}", inventory.money);
    }
}

fn reset_money(mut inv: ResMut<Inventory>) {
    inv.money = DEFAULT_PURSE_SIZE;
}

fn teardown_debug_ui(mut commands: Commands, nodes: Query<Entity, With<InventoryDebugUI>>) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}
