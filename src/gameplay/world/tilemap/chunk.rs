use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    gameplay::{
        sprite_sort::ZIndexSprite,
        world::{
            WorldSpawnSystems,
            tilemap::{CHUNK_SIZE, TILE_OFFSET, TILE_SIZE},
        },
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.register_type::<Chunk>();
    app.register_type::<Layers>();
    app.register_type::<LayerOf>();

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

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = LayerOf)]
pub struct Layers(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = Layers)]
pub struct LayerOf(pub Entity);

fn spawn_chunk(mut commands: Commands, asset_server: Res<AssetServer>) {
    let chunk_id = commands
        .spawn((
            Name::new("Chunk"),
            Chunk,
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    for i in 0..4 {
        let map_size = TilemapSize::from(CHUNK_SIZE);
        let tile_size = TilemapTileSize::from(TILE_SIZE);
        let grid_size = TilemapGridSize::from(TILE_OFFSET);
        let storage = TileStorage::empty(map_size);

        commands.spawn((
            Name::new(format!("Layer {i}")),
            LayerOf(chunk_id),
            ChildOf(chunk_id),
            ZIndexSprite(i),
            TilemapBundle {
                transform: Transform::from_xyz(0.0, grid_size.y * i as f32, 0.0),
                grid_size,
                size: map_size,
                storage,
                texture: TilemapTexture::Vector(vec![
                    asset_server.load("tiles/grass.png"),
                    asset_server.load("tiles/path.png"),
                ]),
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
}

fn spawn_flat_ground(
    mut commands: Commands,
    chunk_layers: Single<&Layers, With<Chunk>>,
    mut tile_storage_query: Query<&mut TileStorage>,
) {
    for (i, layer) in chunk_layers.iter().enumerate().take(4) {
        let Ok(mut tile_storage) = tile_storage_query.get_mut(layer) else {
            continue;
        };

        fill_tilemap_rect(
            TileTextureIndex(i as u32 % 2),
            TilePos::new(0, 0),
            TilemapSize::from(CHUNK_SIZE / (i + 1) as u32),
            TilemapId(layer),
            &mut commands,
            &mut tile_storage,
        );
    }
}
