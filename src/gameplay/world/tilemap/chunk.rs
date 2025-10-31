use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    gameplay::{
        sprite_sort::ZIndexSprite,
        world::tilemap::{CHUNK_SIZE, TILE_OFFSET, TILE_SIZE},
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.init_resource::<ChunkManager>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_world);

    app.add_systems(Update, (spawn_chunks_around_camera, despawn_chunks));
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashMap<IVec2, Entity>,
}

#[derive(Event, Reflect, Debug)]
pub struct ChunkLoaded {
    pub chunk: Entity,
}

#[derive(Event, Reflect, Debug)]
pub struct ChunkUnloaded {
    pub chunk: Entity,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct World;

#[derive(Component, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Chunk(pub IVec2);

const CHUNK_RENDER_DISTANCE: UVec2 = UVec2::new(3, 3);

const CHUNK_SIZE_PIXELS: Vec2 = Vec2::new(
    CHUNK_SIZE.x as f32 * TILE_OFFSET.x,
    CHUNK_SIZE.y as f32 * TILE_OFFSET.y,
);

pub const CHUNK_MATRIX: Mat2 = Mat2::from_cols(
    Vec2::new(CHUNK_SIZE_PIXELS.x * 0.5, -(CHUNK_SIZE_PIXELS.y * 0.5)),
    Vec2::new(CHUNK_SIZE_PIXELS.x * 0.5, CHUNK_SIZE_PIXELS.y * 0.5),
);

pub fn translation_to_chunk(translation: &Vec2) -> Chunk {
    let ivec = (CHUNK_MATRIX.inverse() * translation).round().as_ivec2();
    Chunk(ivec)
}

pub fn chunk_to_translation(chunk: &Chunk) -> Vec2 {
    CHUNK_MATRIX * chunk.0.as_vec2()
}

fn spawn_world(mut commands: Commands) {
    commands.spawn((
        Name::new("World"),
        World,
        Transform::default(),
        Visibility::default(),
    ));
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    camera_transform: Single<&Transform, With<Camera>>,
    asset_server: Res<AssetServer>,
    mut chunk_manager: ResMut<ChunkManager>,
    world: Single<Entity, With<World>>,
) {
    let focused_chunk = translation_to_chunk(&camera_transform.translation.xy());

    for y in (focused_chunk.y - CHUNK_RENDER_DISTANCE.y as i32)
        ..=(focused_chunk.y + CHUNK_RENDER_DISTANCE.y as i32)
    {
        for x in (focused_chunk.x - CHUNK_RENDER_DISTANCE.x as i32)
            ..=(focused_chunk.x + CHUNK_RENDER_DISTANCE.x as i32)
        {
            let chunk_pos = IVec2::new(x, y);

            if chunk_manager.spawned_chunks.contains_key(&chunk_pos) {
                continue;
            }

            let chunk = commands.spawn_empty().id();
            let map_size = TilemapSize::from(CHUNK_SIZE);
            let tile_size = TilemapTileSize::from(TILE_SIZE);
            let grid_size = TilemapGridSize::from(TILE_OFFSET);
            let mut storage = TileStorage::empty(map_size);

            fill_tilemap_rect(
                TileTextureIndex(0),
                TilePos::new(0, 0),
                TilemapSize::from(CHUNK_SIZE),
                TilemapId(chunk),
                &mut commands,
                &mut storage,
            );

            let chunk_translation = chunk_to_translation(&Chunk(chunk_pos));
            let transform = Transform::from_translation(chunk_translation.extend(0.0));

            commands.entity(chunk).insert((
                Name::new("Chunk"),
                Chunk(chunk_pos),
                ChildOf(*world),
                ZIndexSprite(0),
                TilemapBundle {
                    transform,
                    grid_size,
                    size: map_size,
                    storage,
                    texture: TilemapTexture::Single(asset_server.load("tiles/grass.png")),
                    tile_size,
                    map_type: TilemapType::Isometric(IsoCoordSystem::Diamond),
                    render_settings: TilemapRenderSettings {
                        render_chunk_size: UVec2::new(map_size.x, 1),
                        y_sort: true,
                    },
                    ..default()
                },
            ));

            chunk_manager.spawned_chunks.insert(chunk_pos, chunk);

            commands.trigger(ChunkLoaded { chunk });
        }
    }
}

fn despawn_chunks(
    mut commands: Commands,
    camera_transform: Single<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    chunk_query: Query<(Entity, &Chunk)>,
) {
    let focused_chunk = translation_to_chunk(&camera_transform.translation.xy());

    for (chunk, chunk_coord) in chunk_query {
        if chunk_coord.x.abs_diff(focused_chunk.x) > CHUNK_RENDER_DISTANCE.x
            || chunk_coord.y.abs_diff(focused_chunk.y) > CHUNK_RENDER_DISTANCE.y
        {
            commands.trigger(ChunkUnloaded { chunk });
            commands.entity(chunk).despawn();
            chunk_manager.spawned_chunks.remove(&chunk_coord.0);
        }
    }
}
