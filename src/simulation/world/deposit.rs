use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::Rng;
use serde::Deserialize;

use crate::{
    assets::{
        LoadResource,
        manifest::{Id, Manifest, ManifestPlugin},
    },
    screens::Screen,
    simulation::{
        item::{ItemDef, PlayerInventory, Stack},
        recipe::RecipeDef,
        world::{MAP_SIZE, Terrain, WorldSpawnSystems},
    },
    ui::{Interact, Interactable, YSort},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(ManifestPlugin::<Deposit>::default())
        .register_type::<DepositAssets>()
        .load_resource::<DepositAssets>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_deposits.in_set(WorldSpawnSystems::SpawnDeposits),
    );

    app.add_observer(on_mine_deposit);
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Deposit {
    name: String,
    recipe_id: String,
    quantity: u32,
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct DepositAssets {
    manifest: Handle<Manifest<Deposit>>,
    aseprite: Handle<Aseprite>,
}

impl FromWorld for DepositAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            manifest: assets.load("manifest/deposits.toml"),
            aseprite: assets.load("deposits.aseprite"),
        }
    }
}

impl DepositAssets {
    fn sprite(&self, id: &Id<Deposit>) -> impl Bundle {
        (
            Sprite::sized(Vec2::splat(64.0)),
            AseSlice {
                aseprite: self.aseprite.clone(),
                name: id.value.clone(),
            },
        )
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DepositRecipe(pub String);

fn spawn_deposits(
    mut commands: Commands,
    terrain: Single<Entity, With<Terrain>>,
    deposit_assets: Res<DepositAssets>,
    deposit_manifests: Res<Assets<Manifest<Deposit>>>,
) {
    let mut rng = rand::rng();

    let deposits = deposit_manifests
        .get(&deposit_assets.manifest)
        .expect("Deposit manifest not loaded");

    for (id, deposit) in deposits.iter() {
        for _ in 0..deposit.quantity {
            commands.spawn((
                Name::new(deposit.name.clone()),
                Transform::from_xyz(
                    rng.random_range(0.0..MAP_SIZE) - MAP_SIZE / 2.0,
                    rng.random_range(0.0..MAP_SIZE) - MAP_SIZE / 2.0,
                    1.0,
                ),
                ChildOf(*terrain),
                YSort(0.1),
                deposit_assets.sprite(id),
                Pickable::default(),
                Interactable,
                DepositRecipe(deposit.recipe_id.clone()),
            ));
        }
    }
}

fn on_mine_deposit(
    trigger: Trigger<Interact>,
    deposits: Query<&DepositRecipe>,
    mut inventory: Single<&mut PlayerInventory>,
    recipes: Res<Assets<RecipeDef>>,
    items: Res<Assets<ItemDef>>,
) {
    let Ok(deposit_recipe) = deposits.get(trigger.target()) else {
        return;
    };

    let recipe_def = recipes
        .iter()
        .map(|(_, recipe_def)| recipe_def)
        .find(|recipe_def| recipe_def.id == deposit_recipe.0)
        .expect("Deposit refers to non-existent recipe");

    for (item_id, quantity) in recipe_def.output.iter() {
        let item_def = items
            .iter()
            .map(|(_, item_def)| item_def)
            .find(|item_def| item_def.id == *item_id)
            .expect("Recipe refers to invalid item id");

        let mut stack = Stack {
            item_id: item_def.id.to_owned(),
            quantity: *quantity,
            max_quantity: item_def.stack_size,
        };

        let _ = inventory.add_stack(&mut stack);
    }
}
