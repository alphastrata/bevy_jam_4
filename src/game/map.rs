use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;

use rand::{thread_rng, Rng};

use crate::AppState;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), create_initial_map)
            .add_systems(
                Update,
                (highlight_tile_labels).run_if(in_state(AppState::Playing)),
            );
    }
}

fn create_initial_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Create initial tilemap");

    let texture = asset_server.load("tiles/temporary-terrain-tiles.png");

    let map_size = TilemapSize { x: 64, y: 64 };

    let mut tile_storage = TileStorage::empty(map_size);

    let tilemap_entity = commands.spawn_empty().id();

    let mut random = thread_rng();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(random.gen_range(0..9)),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

/// Highlight visualisation on tile hover
fn highlight_tile_labels(
    _commands: Commands,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&GlobalTransform, &Camera)>,
    q_tilemap: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    // highlighted_tiles_q: Query<Entity, With<HighlightedLabel>>,
    // tile_label_q: Query<&TileLabel>,
) {
    let window = primary_window.single();
    let (cam_tf, cam) = q_camera.single();
    let (map_size, grid_size, map_type, tile_storage, map_transform) = q_tilemap.single();

    let cursor_world_space_pos = window
        .cursor_position()
        .and_then(|viewport_pos| {
            // warn!("World pos: {:?}", world);
            cam.viewport_to_world_2d(cam_tf, viewport_pos)
        })
        .map(|cursor_world| {
            // need to account for the maps transform

            let cursor_pos = Vec4::from((cursor_world, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            // warn!("Map pos: {:?}", cursor_in_map_pos.xy());
            cursor_in_map_pos.xy()
        })
        .and_then(|world_pos| TilePos::from_world_pos(&world_pos, map_size, grid_size, map_type));

    if let Some(tile_pos) = cursor_world_space_pos {
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            trace!("Hovered over Tile {:?} entity {:?}", tile_pos, tile_entity);
        }
    }
}

// TODO
fn hide_map(mut _commands: Commands) {}
