use bevy::{asset::LoadedFolder, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use rand::Rng;
use serde::Deserialize;

use crate::{
    assets::{loaders::toml::TomlAssetPlugin, tracking::LoadResource},
    gameplay::{
        sprite_sort::{YSortSprite, ZIndexSprite},
        world::{
            WorldSpawnSystems,
            construction::Constructions,
            tilemap::{CHUNK_SIZE, TILE_SIZE, coord::Coord},
        },
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<DepositDef>::extensions(&["deposit.toml"]));

    app.load_resource::<DepositAssets>();

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

fn spawn_deposits(
    mut commands: Commands,
    deposit_definitions: Res<Assets<DepositDef>>,
    asset_server: Res<AssetServer>,
    mut constructions: ResMut<Constructions>,
) {
    let mut rng = rand::rng();

    for (_, deposit) in deposit_definitions.iter() {
        for _ in 0..deposit.quantity {
            let tile_pos = TilePos::new(
                rng.random_range(0..CHUNK_SIZE.x),
                rng.random_range(0..CHUNK_SIZE.y),
            );

            let entity = commands
                .spawn((
                    Name::new(deposit.name.clone()),
                    Coord::new(tile_pos.x, tile_pos.y),
                    YSortSprite,
                    ZIndexSprite(10),
                    Sprite {
                        image: asset_server.load(format!("sprites/deposits/{}.png", deposit.id)),
                        custom_size: Vec2::new(TILE_SIZE.x, TILE_SIZE.y).into(),
                        ..default()
                    },
                    DepositRecipe(deposit.recipe_id.clone()),
                ))
                .id();

            constructions.insert(tile_pos.into(), entity);
        }
    }
}
