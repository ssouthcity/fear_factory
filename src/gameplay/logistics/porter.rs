use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

use crate::gameplay::{
    FactorySystems,
    item::{
        assets::{ItemDef, Transport},
        inventory::Inventory,
    },
    logistics::pathfinding::{PorterPaths, WalkPath},
    recipe::{assets::Recipe, select::SelectedRecipe},
    sprite_sort::{YSortSprite, ZIndexSprite},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PorterArrival>();
    app.add_message::<PorterLost>();

    app.add_systems(
        FixedUpdate,
        (spawn_porter, despawn_lost_porters, drop_of_items).in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
#[require(PorterSpawnOutputIndex)]
pub struct PorterSpawnTimer(pub Timer);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
struct PorterSpawnOutputIndex(usize);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = PorterOf)]
pub struct Porters(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Porters)]
pub struct PorterOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PorterTo(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PortingItem(pub AssetId<ItemDef>);

#[derive(Message, Reflect, Debug)]
pub struct PorterArrival(pub Entity);

#[derive(Message, Reflect, Debug)]
pub struct PorterLost(pub Entity);

fn spawn_porter(
    structure_query: Query<(
        Entity,
        &Transform,
        &mut PorterSpawnTimer,
        &mut PorterSpawnOutputIndex,
        &SelectedRecipe,
        &mut Inventory,
        &mut PorterPaths,
    )>,
    mut commands: Commands,
    item_defs: Res<Assets<ItemDef>>,
    recipes: Res<Assets<Recipe>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (
        structure,
        transform,
        mut timer,
        mut index,
        selected_recipe,
        mut inventory,
        mut porter_paths,
    ) in structure_query
    {
        if !timer.tick(time.delta()).is_finished() {
            continue;
        }

        let Some(recipe) = recipes.get(&selected_recipe.0) else {
            continue;
        };

        let Some((item_id, _)) = recipe.output.iter().nth(index.0) else {
            continue;
        };

        let Some(quantity) = inventory.items.get_mut(item_id) else {
            continue;
        };

        if *quantity == 0 {
            continue;
        }

        let Some((input, path)) = porter_paths.0.front() else {
            continue;
        };

        let Some(item_def) = item_defs.get(*item_id) else {
            continue;
        };

        commands.spawn((
            Name::new("Porter"),
            *transform,
            Sprite::default(),
            Anchor(Vec2::new(0.0, -0.25)),
            AseAnimation {
                aseprite: asset_server.load("sprites/logistics/porter.aseprite"),
                animation: match item_def.transport {
                    Transport::Box => Animation::tag("walk_item"),
                    Transport::Bag => Animation::tag("walk_bag"),
                },
            },
            Inventory::with_single(*item_id, 1),
            YSortSprite,
            ZIndexSprite(10),
            PorterOf(structure),
            PorterTo(*input),
            PortingItem(*item_id),
            WalkPath(path.clone()),
        ));

        *quantity -= 1;
        index.0 = (index.0 + 1) % recipe.output.len();
        porter_paths.0.rotate_left(1);
        timer.reset();
    }
}

fn despawn_lost_porters(mut porter_losses: MessageReader<PorterLost>, mut commands: Commands) {
    for PorterLost(entity) in porter_losses.read() {
        commands.entity(*entity).despawn();
    }
}

fn drop_of_items(
    mut poter_arrivals: MessageReader<PorterArrival>,
    porters: Query<(&PortingItem, &PorterTo)>,
    mut structures: Query<&mut Inventory>,
    mut commands: Commands,
) {
    for PorterArrival(porter) in poter_arrivals.read() {
        commands.entity(*porter).despawn();

        let (PortingItem(item_id), PorterTo(structure)) = porters.get(*porter).unwrap();

        if let Ok(mut inventory) = structures.get_mut(*structure) {
            inventory
                .items
                .entry(*item_id)
                .and_modify(|q| *q += 1)
                .or_insert(1);
        }
    }
}
