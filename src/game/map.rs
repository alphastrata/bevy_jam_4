use bevy::{prelude::*, transform::commands, utils::HashMap, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;
use image::{GrayImage, ImageFormat, Luma};

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

use super::{camera::ViewCamera, placement::PlacementState};
use crate::{AppState, Teardown};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTileHover>()
            .add_systems(
                OnEnter(AppState::Gameplay),
                (create_initial_map2, setup_highlight_tile),
            )
            .add_systems(
                Update,
                (highlight_tile_labels).run_if(in_state(AppState::Gameplay)),
            );
    }
}

#[derive(Component)]
struct HighlightedTile;

#[derive(Resource, Default)]
pub struct CurrentTileHover {
    pub tile_pos: Option<TilePos>,
    pub world_pos: Option<Vec2>,
}

enum TerrainTileType {
    Dirt,
    Grass,
    Rock,
    Sand,
}

lazy_static! {
    static ref TERRAIN_TILE_TYPE_TO_INDICIE_MAP: HashMap<usize, TerrainTileType> = {
        let mut m = HashMap::new();
        m.insert(0usize, TerrainTileType::Sand);
        m.insert(1usize, TerrainTileType::Sand);

        m.insert(2usize, TerrainTileType::Dirt);
        m.insert(3usize, TerrainTileType::Dirt);

        m.insert(4usize, TerrainTileType::Grass);
        m.insert(5usize, TerrainTileType::Grass);

        m.insert(6usize, TerrainTileType::Grass);
        m.insert(7usize, TerrainTileType::Dirt);
        m.insert(8usize, TerrainTileType::Rock);

        m
    };
}

pub fn create_initial_map2(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Create initial tilemap");

    // Convert the image to grayscale
    let greyscale_img = brightness_map();
    let (height, width) = greyscale_img.dimensions();

    let brightness_map: Vec<Vec<u8>> = (0..width)
        .map(|y| {
            (0..height)
                .map(|x| greyscale_img.get_pixel(x, y)[0])
                .collect()
        })
        .collect();

    let texture = asset_server.load("textures/terrain.png");
    let map_size = TilemapSize {
        y: brightness_map.first().unwrap().len() as u32,
        x: brightness_map.len() as u32,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    brightness_map
        .into_iter()
        .enumerate()
        .for_each(|(x_idx, ixs)| {
            ixs.iter().enumerate().for_each(|(y_idx, brightness)| {
                let tile_pos = TilePos {
                    x: x_idx as u32,
                    y: y_idx as u32,
                };

                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(match *brightness % 8u8 {
                            0 => 6,
                            1 => 6,
                            2 => 5,
                            3 => 4,
                            4 => 4,
                            5 => 4,
                            6 => 3,
                            7 => 5,
                            _ => 0,
                        } as u32),
                        ..Default::default()
                    })
                    .id();

                tile_storage.set(&tile_pos, tile_entity);
            });
        });

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands
        .entity(tilemap_entity)
        .insert(Teardown)
        .insert(TilemapBundle {
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

fn brightness_map() -> GrayImage {
    let static_map: &[u8] = include_bytes!("../../assets/textures/noise3.png");
    image::load_from_memory_with_format(static_map, ImageFormat::Png)
        .expect("Failed to load static-map.png!")
        .to_luma8()
}

// NOTE: this code was cool so I left it here for us to admire :)

// /// Make a perlin-noise based brightnessmap:
// #[deprecated = "We're using a static map from a known .png embedded into the binary"]
// fn brightness_map2() -> GrayImage {
// let size = 512;
// let mut img = GrayImage::new(size, size);
// let perlin = Perlin::new(1);
//
// let center = (size as f32 / 2.0, size as f32 / 2.0);
// let radius = size as f32 / 64.0; // Adjust radius as needed
//
// (0..size).for_each(|x| {
//     (0..size).for_each(|y| {
//         let nx = x as f32 / size as f32 - 0.5;
//         let ny = y as f32 / size as f32 - 0.5;
//
//         let noise_value = perlin.get([nx as f64, ny as f64, 0.0]) as f32;
//         let pixel_value = ((noise_value + 1.0) / 2.0 * 255.0) as u8;
//
//         // Apply a circular mask
//         let dist_from_center =
//             ((x as f32 - center.0).powi(2) + (y as f32 - center.1).powi(2)).sqrt();
//
//         let masked_value = if dist_from_center < radius {
//             6
//         } else {
//             pixel_value
//         };
//
//         img.put_pixel(x, y, Luma([masked_value]));
//     });
// });
//
// img
// }

/// Highlight visualisation on tile hover
fn highlight_tile_labels(
    mut highlight_rect: Query<(Entity, &mut Transform, &mut Visibility), With<TheHighlightRect>>,
    mut tile_hover: ResMut<CurrentTileHover>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&GlobalTransform, &Camera), With<ViewCamera>>,
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
    _placement: Res<PlacementState>,
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

    let (_, mut hr, mut _rect_vis) = highlight_rect.single_mut();

    if let Some(tile_pos) = cursor_world_space_pos {
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            trace!("Hovered over Tile {:?} entity {:?}", tile_pos, tile_entity);

            let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);

            let transform = *map_transform * Transform::from_translation(tile_center);

            let tile_top_left = Vec2::new(transform.translation.x, transform.translation.y)
                + Vec2::new(0.0, 32.0 / 2.0);
            tile_hover.tile_pos = Some(tile_pos);
            tile_hover.world_pos = Some(tile_top_left);

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
            visibility: Visibility::Hidden, // Temp
            ..default()
        },
        TheHighlightRect,
        Teardown,
    ));
}
