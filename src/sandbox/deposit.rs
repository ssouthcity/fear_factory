use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::Rng;

use crate::{
    assets::manifest::Id,
    item::Item,
    sandbox::{COAL_DEPOSITS, IRON_DEPOSITS, SANDBOX_MAP_SIZE, Sandbox, SandboxSpawnSystems},
    ui::YSort,
};

pub fn plugin(app: &mut App) {
    app.register_type::<DepositAssets>();

    app.init_resource::<DepositAssets>();

    app.add_systems(Startup, load_deposit_assets);

    app.add_systems(
        Startup,
        spawn_deposits
            .after(load_deposit_assets)
            .in_set(SandboxSpawnSystems::SpawnDeposits),
    );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DepositAssets {
    aseprite: Handle<Aseprite>,
}

impl DepositAssets {
    fn sprite(&self, item_id: Id<Item>) -> impl Bundle {
        (
            Sprite::sized(Vec2::splat(64.0)),
            AseSlice {
                aseprite: self.aseprite.clone(),
                name: match item_id.id.as_str() {
                    "coal" => "coal deposit".to_string(),
                    "iron_ore" => "iron ore deposit".to_string(),
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
pub struct Deposit(pub Id<Item>);

fn spawn_deposits(
    mut commands: Commands,
    deposit_assets: Res<DepositAssets>,
    sandbox: Single<Entity, With<Sandbox>>,
) {
    let mut rng = rand::rng();

    for _ in 0..COAL_DEPOSITS {
        commands.spawn((
            Name::new("Coal Deposit"),
            Transform::from_xyz(
                rng.random_range(0.0..SANDBOX_MAP_SIZE) - SANDBOX_MAP_SIZE / 2.0,
                rng.random_range(0.0..SANDBOX_MAP_SIZE) - SANDBOX_MAP_SIZE / 2.0,
                1.0,
            ),
            YSort(0.1),
            ChildOf(*sandbox),
            deposit_assets.sprite("coal".into()),
            Pickable::default(),
            Deposit("coal".into()),
        ));
    }

    for _ in 0..IRON_DEPOSITS {
        commands.spawn((
            Name::new("Iron Deposit"),
            Transform::from_xyz(
                rng.random_range(0.0..SANDBOX_MAP_SIZE) - SANDBOX_MAP_SIZE / 2.0,
                rng.random_range(0.0..SANDBOX_MAP_SIZE) - SANDBOX_MAP_SIZE / 2.0,
                1.0,
            ),
            YSort(0.1),
            ChildOf(*sandbox),
            deposit_assets.sprite("iron_ore".into()),
            Pickable::default(),
            Deposit("iron_ore".into()),
        ));
    }
}
