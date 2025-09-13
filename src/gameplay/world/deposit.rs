use bevy::{asset::LoadedFolder, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use rand::Rng;
use serde::Deserialize;

use crate::{
    assets::{loaders::toml::TomlAssetPlugin, tracking::LoadResource},
    gameplay::{
        structure::interactable::Interactable,
        world::{MAP_SIZE, WorldSpawnSystems, terrain::Worldly},
        y_sort::YSort,
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.register_type::<DepositAssets>()
        .load_resource::<DepositAssets>();

    app.add_plugins(TomlAssetPlugin::<DepositDef>::extensions(&["deposit.toml"]));

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
    deposit_assets: Res<DepositAssets>,
    deposit_definitions: Res<Assets<DepositDef>>,
) {
    let mut rng = rand::rng();

    for (_, deposit) in deposit_definitions.iter() {
        for _ in 0..deposit.quantity {
            commands.spawn((
                Name::new(deposit.name.clone()),
                Transform::from_xyz(
                    rng.random_range(0.0..MAP_SIZE) - MAP_SIZE / 2.0,
                    rng.random_range(0.0..MAP_SIZE) - MAP_SIZE / 2.0,
                    1.0,
                ),
                Worldly,
                YSort(0.1),
                Sprite::sized(Vec2::splat(64.0)),
                AseSlice {
                    aseprite: deposit_assets.aseprite.clone(),
                    name: deposit.id.clone(),
                },
                Pickable::default(),
                Interactable,
                DepositRecipe(deposit.recipe_id.clone()),
            ));
        }
    }
}
