use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::Rng;
use serde::Deserialize;

use crate::{
    assets::{
        LoadResource,
        manifest::{Id, Manifest, ManifestPlugin},
    },
    item::{Item, ItemAssets, PlayerInventory, Stack},
    sandbox::{SANDBOX_MAP_SIZE, Sandbox, SandboxSpawnSystems},
    screens::Screen,
    ui::{Interact, Interactable, YSort},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(ManifestPlugin::<Deposit>::default())
        .register_type::<DepositAssets>()
        .load_resource::<DepositAssets>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_deposits.in_set(SandboxSpawnSystems::SpawnDeposits),
    );

    app.add_observer(on_mine_deposit);
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Deposit {
    name: String,
    item_id: Id<Item>,
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
pub struct DepositItem(pub Id<Item>);

fn spawn_deposits(
    mut commands: Commands,
    sandbox: Single<Entity, With<Sandbox>>,
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
                    rng.random_range(0.0..SANDBOX_MAP_SIZE) - SANDBOX_MAP_SIZE / 2.0,
                    rng.random_range(0.0..SANDBOX_MAP_SIZE) - SANDBOX_MAP_SIZE / 2.0,
                    1.0,
                ),
                ChildOf(*sandbox),
                YSort(0.1),
                deposit_assets.sprite(id),
                Pickable::default(),
                Interactable::default(),
                DepositItem(deposit.item_id.clone()),
            ));
        }
    }
}

fn on_mine_deposit(
    trigger: Trigger<Interact>,
    deposits: Query<&DepositItem>,
    mut inventory: Single<&mut PlayerInventory>,
    item_manifests: Res<Assets<Manifest<Item>>>,
    item_assets: Res<ItemAssets>,
) {
    let items = item_manifests
        .get(&item_assets.manifest)
        .expect("Item manifest not loaded");

    let Ok(deposit) = deposits.get(trigger.target()) else {
        return;
    };

    let Some(ore_definition) = items.get(&deposit.0) else {
        return;
    };

    let mut stack = Stack::from(ore_definition).with_quantity(1);

    let _ = inventory.add_stack(&mut stack);
}
