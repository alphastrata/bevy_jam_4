use bevy::{prelude::*, transform::commands, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;

use rand::{thread_rng, Rng};

use crate::AppState;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Gameplay),
            (create_initial_map, setup_highlight_tile),
        )
        .add_systems(
            Update,
            (highlight_tile_labels).run_if(in_state(AppState::Gameplay)),
        );
    }
}

#[derive(Component)]
struct HighlightedTile;

fn create_initial_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Create initial tilemap");

    let texture = asset_server.load("tiles/temporary-terrain-tiles.png");

    let map_size = TilemapSize { x: 512, y: 512 };

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
    q_tilemap: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
        ),
        Without<TheHighlightRect>,
    >,
    mut highlight_rect: Query<(Entity, &mut Transform), With<TheHighlightRect>>,
) {
    let window = primary_window.single();
    let (cam_tf, cam) = q_camera.single();
    let (map_size, grid_size, map_type, tile_storage, map_transform) = q_tilemap.single();

    let cursor_world_space_pos = window
        .cursor_position()
        .and_then(|viewport_pos| cam.viewport_to_world_2d(cam_tf, viewport_pos))
        .map(|world_pos| {
            let cursor_pos = Vec4::from((world_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        })
        .and_then(|map_pos| TilePos::from_world_pos(&map_pos, map_size, grid_size, map_type));

    let (_, mut hr) = highlight_rect.single_mut();

    if let Some(tile_pos) = cursor_world_space_pos {
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            trace!("Hovered over Tile {:?} entity {:?}", tile_pos, tile_entity);

            let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
            let transform = *map_transform * Transform::from_translation(tile_center);
            *hr = transform;
        }
    } else {
        *hr = Transform::default();
    }
}

/// SKUX way of doing things but I'm really tired. Keep an omnipresent rectangle
/// around and just move it on top of the hovered tile. It's just an Entity
#[derive(Component)]
struct TheHighlightRect;

fn setup_highlight_tile(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.85, 0.2, 0.3),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::default(),
            ..default()
        },
        TheHighlightRect,
    ));
}
