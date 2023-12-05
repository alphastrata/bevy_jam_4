use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use rand::{thread_rng, Rng};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_initial_map);
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
            // Here we let the tile storage component know what tiles we have.
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
