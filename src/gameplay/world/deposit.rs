use bevy::{asset::LoadedFolder, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::{
    anchor::TilemapAnchor,
    map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use rand::Rng;
use serde::Deserialize;

use crate::{
    assets::{loaders::toml::TomlAssetPlugin, tracking::LoadResource},
    gameplay::{
        sprite_sort::{YSortSprite, ZIndexSprite},
        world::{
            WorldSpawnSystems,
            tilemap::{
                CHUNK_SIZE,
                chunk::{Chunk, Layers},
            },
        },
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<DepositDef>::extensions(&["deposit.toml"]));

    app.register_type::<DepositAssets>()
        .load_resource::<DepositAssets>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_deposits.in_set(WorldSpawnSystems::SpawnDeposits),
    );
}

#[derive(Asset, Deserialize, Reflect)]
pub struct DepositDef {
    pub id: String,
    pub name: String,
    pub recipe_id: String,
    pub quantity: u32,
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct DepositAssets {
    aseprite: Handle<Aseprite>,
    manifest_folder: Handle<LoadedFolder>,
}

impl FromWorld for DepositAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            aseprite: assets.load("deposits.aseprite"),
            manifest_folder: assets.load_folder("manifests/deposits"),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DepositRecipe(pub String);

fn spawn_deposits(
    mut commands: Commands,
    // deposit_assets: Res<DepositAssets>,
    deposit_definitions: Res<Assets<DepositDef>>,
    asset_server: Res<AssetServer>,
    chunk_query: Single<&Layers, With<Chunk>>,
    tile_storage_query: Query<&TileStorage>,
    tilemap_query: Query<(
        &Transform,
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    let mut rng = rand::rng();

    for (_, deposit) in deposit_definitions.iter() {
        for _ in 0..deposit.quantity {
            let tile_pos = TilePos::new(
                rng.random_range(0..CHUNK_SIZE.x),
                rng.random_range(0..CHUNK_SIZE.y),
            );

            let Some((z_index, layer)) = chunk_query.iter().enumerate().find(|(_, layer)| {
                let storage = tile_storage_query.get(*layer).unwrap();
                storage.get(&tile_pos).is_none()
            }) else {
                continue;
            };

            let (transform, map_size, grid_size, tile_size, map_type, anchor) =
                tilemap_query.get(layer).unwrap();

            let translation = tile_pos
                .center_in_world(map_size, grid_size, tile_size, map_type, anchor)
                .extend(0.0);

            commands.spawn((
                Name::new(deposit.name.clone()),
                Transform::from_translation(transform.translation + translation),
                Sprite::from_image(asset_server.load("tiles/iron_deposit.png")),
                YSortSprite,
                ZIndexSprite(z_index as u32),
                DepositRecipe(deposit.recipe_id.clone()),
            ));
        }
    }
}
