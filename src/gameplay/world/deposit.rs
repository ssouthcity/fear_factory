use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*, sprite::Anchor};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use serde::Deserialize;

use crate::{
    assets::{loaders::toml::TomlAssetPlugin, tracking::LoadResource},
    gameplay::{
        sprite_sort::{YSortSprite, ZIndexSprite},
        world::{
            construction::Constructions,
            tilemap::{
                CHUNK_SIZE, TILE_SIZE,
                chunk::{Chunk, ChunkLoaded, ChunkUnloaded},
                coord::Coord,
            },
        },
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<DepositDef>::extensions(&["deposit.toml"]));
    app.load_resource::<DepositAssets>();

    app.init_resource::<DepositNoise>();
    app.add_systems(OnEnter(Screen::Gameplay), create_noise);

    app.add_observer(spawn_deposits);
    app.add_observer(unload_deposits);
}

#[derive(Asset, Deserialize, Reflect)]
pub struct DepositDef {
    pub id: String,
    pub name: String,
    pub recipe_id: String,
    pub seed: u32,
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct DepositAssets {
    sprites: Handle<LoadedFolder>,
    manifest_folder: Handle<LoadedFolder>,
}

impl FromWorld for DepositAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            sprites: assets.load_folder("sprites/deposits"),
            manifest_folder: assets.load_folder("manifests/deposits"),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DepositRecipe(pub String);

#[derive(Resource, Debug, Default)]
pub struct DepositNoise {
    pub noises: HashMap<AssetId<DepositDef>, Fbm<Perlin>>,
}

fn create_noise(
    mut deposit_noise: ResMut<DepositNoise>,
    deposit_definitions: Res<Assets<DepositDef>>,
) {
    for (deposit_id, deposit_def) in deposit_definitions.iter() {
        let fbm = Fbm::<Perlin>::new(deposit_def.seed)
            .set_octaves(5)
            .set_frequency(1.0 / 70.0)
            .set_lacunarity(2.5)
            .set_persistence(0.45);

        deposit_noise.noises.insert(deposit_id, fbm);
    }
}

fn spawn_deposits(
    chunk_loaded: On<ChunkLoaded>,
    chunk_query: Query<&Chunk>,
    mut commands: Commands,
    deposit_definitions: Res<Assets<DepositDef>>,
    asset_server: Res<AssetServer>,
    deposit_noise: Res<DepositNoise>,
    mut constructions: ResMut<Constructions>,
) {
    let chunk = chunk_query.get(chunk_loaded.chunk).unwrap();

    let absolute_chunk_position = chunk.0 * CHUNK_SIZE.as_ivec2();

    for (deposit_id, deposit_def) in deposit_definitions.iter() {
        let Some(fbm) = deposit_noise.noises.get(&deposit_id) else {
            return;
        };

        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let absolute_tile_pos = absolute_chunk_position + ivec2(x as i32, y as i32);
                if constructions.contains_key(&absolute_tile_pos) {
                    continue;
                }

                let value = fbm.get(absolute_tile_pos.as_dvec2().into());
                let normalized_value = (value + 1.0) / 2.0;
                if normalized_value < 0.75 {
                    continue;
                }

                let entity = commands
                    .spawn((
                        Name::new(deposit_def.name.clone()),
                        Coord(absolute_tile_pos),
                        Anchor(Vec2::new(0.0, -0.25)),
                        YSortSprite,
                        ZIndexSprite(10),
                        Sprite {
                            image: asset_server
                                .load(format!("sprites/deposits/{}.png", deposit_def.id)),
                            custom_size: Vec2::new(TILE_SIZE.x, TILE_SIZE.y).into(),
                            ..default()
                        },
                        DepositRecipe(deposit_def.recipe_id.clone()),
                    ))
                    .id();

                constructions.insert(absolute_tile_pos, entity);
            }
        }
    }
}

fn unload_deposits(
    chunk_unloaded: On<ChunkUnloaded>,
    chunk_query: Query<&Chunk>,
    mut constructions: ResMut<Constructions>,
    mut commands: Commands,
) {
    let chunk = chunk_query.get(chunk_unloaded.chunk).unwrap();

    let absolute_chunk_position = chunk.0 * CHUNK_SIZE.as_ivec2();

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let absolute_tile_pos = absolute_chunk_position + ivec2(x as i32, y as i32);

            let Some(construction) = constructions.get(&absolute_tile_pos) else {
                continue;
            };

            commands.entity(*construction).despawn();
            constructions.remove(&absolute_tile_pos);
        }
    }
}
