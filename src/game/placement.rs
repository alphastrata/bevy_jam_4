use crate::{
    buildings::{
        distribution::DistributionTower, spawn_building, BuildingDefinition, BuildingType,
    },
    game::{camera::CameraState, power::AddBuilding},
    global_systems::eargasm::AudioRequest,
    AppState, Teardown,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_mod_picking::prelude::*;

use super::{
    camera::ViewCamera,
    keybinds::FloraCommand,
    map::CurrentTileHover,
    resources::{ExpendResource, Inventory, ResourceType},
};

#[derive(Resource, Default)]
pub struct PlacementState {
    /// Some indicates current type of building user has selected in the UI or via keybinding
    /// to place; None indicates no placement is in action. TODO: should disable Tile Highlighting as well
    being_placed_building_type: Option<BuildingType>,
}

/// Spawn towers when clicked
pub struct TowerPlacementPlugin;
impl Plugin for TowerPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlacementState>()
            .add_plugins(DefaultPickingPlugins)
            .add_systems(OnEnter(AppState::Gameplay), setup_ghost_tower)
            .add_systems(
                Update,
                (
                    change_current_building,
                    spawn_at_click_pos,
                    update_ghost_tower,
                )
                    .run_if(in_state(AppState::Gameplay)),
            );
    }
}

fn change_current_building(mut state: ResMut<PlacementState>, input: Res<Input<FloraCommand>>) {
    if input.just_released(FloraCommand::SetPlaceDistributionTower) {
        state.being_placed_building_type = Some(BuildingType::Distribution);
    }
    if input.just_released(FloraCommand::SetPlaceRadarTower) {
        state.being_placed_building_type = Some(BuildingType::Radar);
    }
    if input.just_released(FloraCommand::SetPlaceDrainTower) {
        state.being_placed_building_type = Some(BuildingType::Drain);
    }
}

#[derive(Component)]
pub struct GhostTower;

fn setup_ghost_tower(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::ZERO),
            visibility: Visibility::Visible, // invisible by default
            ..default()
        },
        GhostTower,
        Teardown,
    ));
}

/// Slightly transparent version of the tower sprite the player is placing
fn update_ghost_tower(
    _commands: Commands,
    placement: Res<PlacementState>,
    asset_server: Res<AssetServer>,
    mut q_ghost: Query<(
        Entity,
        &mut Sprite,
        &mut Handle<Image>,
        &GhostTower,
        &mut Transform,
    )>,
    hover_tile: Res<CurrentTileHover>,
) {
    let (_, mut sprite, mut texture, _, mut transform) = q_ghost.single_mut();

    // update position
    if let Some(tile_world_pos) = hover_tile.world_pos {
        transform.translation = Vec3::new(tile_world_pos.x, tile_world_pos.y, 0.4);
    }

    // update what its showing
    if placement.is_changed() {
        match &placement.being_placed_building_type {
            Some(building_type) => {
                sprite.custom_size = Some(Vec2::new(32.0, 64.0));
                let tex: Handle<Image> = asset_server.load(building_type.sprite());
                *texture = tex;
            }
            None => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_at_click_pos(
    mut commands: Commands,
    mut add_building: EventWriter<AddBuilding>,
    mut expend_resource: EventWriter<ExpendResource>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    state: Res<PlacementState>,
    mouse_btns: Res<Input<MouseButton>>,
    audio_mngr: EventWriter<AudioRequest>,
    inventory: Res<Inventory>,
    tile_hover: Res<CurrentTileHover>,
) {
    if mouse_btns.just_pressed(MouseButton::Right) {
        if let Some(building) = &state.being_placed_building_type {
            if let Some(tile_world_pos) = tile_hover.world_pos {
                if inventory.money > building.cost() {
                    building.spawn(
                        &mut commands,
                        texture_atlases,
                        asset_server,
                        tile_world_pos,
                        audio_mngr,
                    );

                    expend_resource.send(ExpendResource(
                        ResourceType::CorporationPoints,
                        building.cost(),
                    ));
                    add_building.send(AddBuilding);
                }
            }
        }
    }
}
