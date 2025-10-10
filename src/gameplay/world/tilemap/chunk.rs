use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    gameplay::world::{
        WorldSpawnSystems,
        tilemap::{CHUNK_SIZE, TILE_OFFSET, TILE_SIZE},
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_chunk, spawn_flat_ground)
            .chain()
            .in_set(WorldSpawnSystems::SpawnMap),
    );
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Chunk;

fn spawn_chunk(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize::from(CHUNK_SIZE);
    let tile_size = TilemapTileSize::from(TILE_SIZE);
    let grid_size = TilemapGridSize::from(TILE_OFFSET);
    let storage = TileStorage::empty(map_size);

    commands.spawn((
        Name::new("Chunk"),
        Chunk,
        TilemapBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            grid_size,
            size: map_size,
            storage,
            texture: TilemapTexture::Single(asset_server.load("tiles/grass.png")),
            tile_size,
            map_type: TilemapType::Isometric(IsoCoordSystem::Diamond),
            anchor: TilemapAnchor::Center,
            render_settings: TilemapRenderSettings {
                render_chunk_size: UVec2::new(map_size.x, 1),
                y_sort: true,
            },
            ..default()
        },
    ));
}

fn spawn_flat_ground(
    mut commands: Commands,
    chunk_query: Query<(Entity, &mut TileStorage), With<Chunk>>,
) {
    for (chunk_entity, mut chunk_tile_storage) in chunk_query {
        fill_tilemap_rect(
            TileTextureIndex(0),
            TilePos::new(0, 0),
            TilemapSize::from(CHUNK_SIZE),
            TilemapId(chunk_entity),
            &mut commands,
            &mut chunk_tile_storage,
        );
    }
}
