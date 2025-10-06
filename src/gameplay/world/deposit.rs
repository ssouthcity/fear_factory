use std::time::Duration;

use bevy::{asset::LoadedFolder, prelude::*, time::common_conditions::on_timer};
use bevy_ecs_tilemap::tiles::TilePos;
use rand::Rng;
use serde::Deserialize;

use crate::{
    assets::{indexing::IndexMap, loaders::toml::TomlAssetPlugin, tracking::LoadResource},
    gameplay::{
        FactorySystems,
        item::{Item, Quantity, assets::ItemDef},
        recipe::{OutputOf, Outputs},
        sprite_sort::{YSortSprite, ZIndexSprite},
        world::{
            WorldSpawnSystems,
            construction::{Constructions, StructureConstructed},
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

    app.add_systems(
        Update,
        (
            attach_harvested_nodes.in_set(FactorySystems::PostConstruction),
            harvest_node
                .in_set(FactorySystems::Harvest)
                .run_if(on_timer(Duration::from_millis(10))),
        ),
    );
}

#[derive(Asset, Deserialize, Reflect)]
pub struct DepositDef {
    pub id: String,
    pub name: String,
    pub item_id: String,
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

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Deposit {
    manifest: Handle<DepositDef>,
    quantity: u32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct HarvestingFrom(pub Vec<Entity>);

fn spawn_deposits(
    mut commands: Commands,
    deposit_definitions: Res<Assets<DepositDef>>,
    asset_server: Res<AssetServer>,
    mut constructions: ResMut<Constructions>,
) {
    let mut rng = rand::rng();

    for (asset_id, deposit) in deposit_definitions.iter() {
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
                    Deposit {
                        manifest: asset_server.get_id_handle(asset_id).unwrap(),
                        quantity: rng.random_range(50..200),
                    },
                ))
                .id();

            constructions.insert(tile_pos.into(), entity);
        }
    }
}

fn attach_harvested_nodes(
    mut structures_constructed: MessageReader<StructureConstructed>,
    structure_query: Query<(&Name, &Coord)>,
    deposit_query: Query<Entity, With<Deposit>>,
    constructions: Res<Constructions>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    item_index: Res<IndexMap<ItemDef>>,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        let Ok((name, coord)) = structure_query.get(*structure) else {
            continue;
        };

        if "Harvester" != name.to_string() {
            continue;
        }

        let neighboring_coords: Vec<UVec2> = vec![
            coord.0.saturating_sub(UVec2::new(0, 1)),
            coord.0.saturating_sub(UVec2::new(1, 0)),
            coord.0.saturating_add(UVec2::new(0, 1)),
            coord.0.saturating_add(UVec2::new(1, 0)),
        ];

        let deposits: Vec<Entity> = neighboring_coords
            .into_iter()
            .filter_map(|c| constructions.get(&c))
            .filter(|e| deposit_query.contains(**e))
            .map(|e| *e)
            .collect();

        commands.entity(*structure).insert(HarvestingFrom(deposits));

        let harvestables = ["flora_a", "fauna_a"];

        for harvestable in harvestables {
            let item_id = item_index.get(harvestable).unwrap();
            let handle = asset_server.get_id_handle(*item_id).unwrap();

            commands.spawn((
                Name::new("Output"),
                Item(handle),
                Quantity(0),
                OutputOf(*structure),
            ));
        }
    }
}

fn harvest_node(
    harvester_query: Query<(&HarvestingFrom, &Outputs)>,
    mut deposit_query: Query<(Entity, &Coord, &mut Deposit)>,
    mut output_query: Query<&mut Quantity, With<OutputOf>>,
    mut commands: Commands,
    mut constructions: ResMut<Constructions>,
) {
    for (harvesting_from, outputs) in harvester_query {
        let Some(node_to_harvest) = harvesting_from.0.first() else {
            continue;
        };

        let Ok((deposit_entity, coord, mut deposit)) = deposit_query.get_mut(*node_to_harvest)
        else {
            continue;
        };

        let Some(output) = outputs.iter().next() else {
            continue;
        };

        let Ok(mut quantity) = output_query.get_mut(output) else {
            continue;
        };

        deposit.quantity = deposit.quantity.saturating_sub(1);
        quantity.0 = quantity.0.saturating_add(1);

        if deposit.quantity == 0 {
            commands.entity(deposit_entity).despawn();
            constructions.remove(coord);
        }
    }
}
