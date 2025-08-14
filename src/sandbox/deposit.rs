use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::Rng;
use serde::Deserialize;

use crate::{
    assets::manifest::{Id, ManifestParam, ManifestPlugin},
    item::{Item, PlayerInventory, Stack},
    sandbox::{SANDBOX_MAP_SIZE, Sandbox, SandboxSpawnSystems},
    screens::Screen,
    ui::{Interact, Interactable, YSort},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(ManifestPlugin::<Deposit>::new("manifest/deposits.toml"));

    app.register_type::<DepositAssets>()
        .init_resource::<DepositAssets>()
        .add_systems(Startup, load_deposit_assets);

    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_deposits
            .after(load_deposit_assets)
            .in_set(SandboxSpawnSystems::SpawnDeposits),
    );

    app.add_observer(on_mine_deposit);
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Deposit {
    name: String,
    item_id: Id<Item>,
    quantity: u32,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DepositAssets {
    aseprite: Handle<Aseprite>,
}

impl DepositAssets {
    fn sprite(&self, item_id: &Id<Deposit>) -> impl Bundle {
        (
            Sprite::sized(Vec2::splat(64.0)),
            AseSlice {
                aseprite: self.aseprite.clone(),
                name: match item_id.id.as_str() {
                    "coal" => "coal deposit".to_string(),
                    "iron" => "iron ore deposit".to_string(),
                    _ => unreachable!("invalid deposit"),
                },
            },
        )
    }
}

fn load_deposit_assets(mut deposit_assets: ResMut<DepositAssets>, asset_server: Res<AssetServer>) {
    deposit_assets.aseprite = asset_server.load("deposits.aseprite");
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DepositItem(pub Id<Item>);

fn spawn_deposits(
    mut commands: Commands,
    deposit_assets: Res<DepositAssets>,
    sandbox: Single<Entity, With<Sandbox>>,
    deposit_manifest: ManifestParam<Deposit>,
) {
    let mut rng = rand::rng();

    let Some(deposits) = deposit_manifest.read() else {
        return;
    };

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
    item_manifest: ManifestParam<Item>,
) {
    let Some(items) = item_manifest.read() else {
        return;
    };

    let Ok(deposit) = deposits.get(trigger.target()) else {
        return;
    };

    let Some(ore_definition) = items.get(&deposit.0) else {
        return;
    };

    let mut stack = Stack::from(&ore_definition);
    stack.quantity = 1;

    let _ = inventory.add_stack(&mut stack);
}
