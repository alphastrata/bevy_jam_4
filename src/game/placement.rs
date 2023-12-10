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
    pub being_placed_building_type: Option<BuildingType>,
}

/// Spawn towers when clicked
pub struct TowerPlacementPlugin;
impl Plugin for TowerPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlacementState>()
            .add_plugins(DefaultPickingPlugins)
            .add_event::<PlacementStateChanged>()
            .add_systems(
                Update,
                (change_current_building, spawn_at_click_pos).run_if(in_state(AppState::Gameplay)),
            );
    }
}

#[derive(Event)]
pub struct PlacementStateChanged {
    pub value: Option<BuildingType>,
}

fn change_current_building(
    mut state: ResMut<PlacementState>,
    input: Res<Input<FloraCommand>>,
    mut changed: EventWriter<PlacementStateChanged>,
) {
    let prev_state = state.being_placed_building_type.clone();
    if input.just_released(FloraCommand::SetPlaceDistributionTower) {
        if state
            .being_placed_building_type
            .as_ref()
            .map_or(false, |val| val == &BuildingType::Distribution)
        {
            state.being_placed_building_type = None;
        } else {
            state.being_placed_building_type = Some(BuildingType::Distribution);
        }
    }
    // if input.just_released(FloraCommand::SetPlaceRadarTower) {
    //     state.being_placed_building_type = Some(BuildingType::Radar);
    // }
    if input.just_released(FloraCommand::SetPlaceDrainTower) {
        if state
            .being_placed_building_type
            .as_ref()
            .map_or(false, |val| val == &BuildingType::Drain)
        {
            state.being_placed_building_type = None;
        } else {
            state.being_placed_building_type = Some(BuildingType::Drain);
        }
    }
    if prev_state != state.being_placed_building_type {
        changed.send(PlacementStateChanged {
            value: state.being_placed_building_type.clone(),
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_at_click_pos(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), (With<CameraState>, With<ViewCamera>)>,
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
        let window = q_window.single();
        let (camera, camera_transform) = q_camera.single();

        // convert viewport pos to worldspace
        if let Some(world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            if let Some(building) = &state.being_placed_building_type {
                building.spawn(
                    &mut commands,
                    texture_atlases,
                    asset_server,
                    world_pos,
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
