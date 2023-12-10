use crate::{
    buildings::{
        distribution::DistributionTower, spawn_building, BuildingDefinition, BuildingType,
    },
    game::{camera::CameraState, power::AddBuilding},
    global_systems::eargasm::AudioRequest,
    AppState,
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
            .add_systems(
                Update,
                (change_current_building, spawn_at_click_pos).run_if(in_state(AppState::Gameplay)),
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
    tile_hover: Res<CurrentTileHover>,
) {
    if mouse_btns.just_pressed(MouseButton::Right) {
        if let Some(building) = &state.being_placed_building_type {
            if let Some(tile_world_pos) = tile_hover.world_pos {
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
