use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        FactorySystems,
        item::{assets::Taxonomy, inventory::Inventory},
        people::{HousedIn, Houses, Person},
        recipe::{assets::Recipe, select::SelectedRecipe},
        structure::range::Range,
        world::{
            construction::{Constructions, StructureConstructed},
            deposit::{Deposit, DepositDef},
            tilemap::coord::Coord,
        },
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            assign_harvester_taxonomy.run_if(on_message::<StructureConstructed>),
            sync_harvester_range,
            assign_harvester_deposit,
            harvest_deposit,
            cleanup_empty_deposits,
        )
            .chain()
            .in_set(FactorySystems::Harvest),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Harvester;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = Harvests)]
pub struct HarvestedBy(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = HarvestedBy)]
#[require(HarvestTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))]
pub struct Harvests(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct HarvestTimer(pub Timer);

fn assign_harvester_taxonomy(
    mut structures_constructed: MessageReader<StructureConstructed>,
    mut harvester_query: Query<(&Coord, &Range, &mut AseAnimation), With<Harvester>>,
    deposit_query: Query<(&Deposit, &Inventory)>,
    deposit_defs: Res<Assets<DepositDef>>,
    constructions: Res<Constructions>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut recipes: ResMut<Assets<Recipe>>,
    recipe_index: Res<IndexMap<Recipe>>,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        let Ok((coord, range, mut ase_animation)) = harvester_query.get_mut(*structure) else {
            continue;
        };

        let Some((deposit, inventory)) = range.iter(coord.0).find_map(|pos| {
            constructions
                .get(&pos)
                .and_then(|d| deposit_query.get(*d).ok())
        }) else {
            continue;
        };

        let Some(deposit_def) = deposit_defs.get(&deposit.0) else {
            continue;
        };

        let Some(item_id) = inventory.items.iter().next().map(|(id, _)| id) else {
            continue;
        };

        let variant = match deposit_def.taxonomy {
            Taxonomy::Flora => "flora",
            Taxonomy::Fauna => "fauna",
            Taxonomy::Minerale => "minerale",
        };

        let Some(recipe) = recipe_index
            .get(&format!("{variant}_a"))
            .and_then(|id| recipes.get_strong_handle(*id))
        else {
            continue;
        };

        commands
            .entity(*structure)
            .insert((SelectedRecipe(recipe), Inventory::with_single(*item_id, 0)));

        ase_animation.aseprite =
            asset_server.load(format!("sprites/structures/harvester_{variant}.aseprite"));
    }
}

fn sync_harvester_range(
    harvester_query: Query<(&mut Range, &Houses), (With<Harvester>, Changed<Houses>)>,
) {
    for (mut range, houses) in harvester_query {
        *range = Range::Diamond(houses.len() as i32);
    }
}

fn assign_harvester_deposit(
    person_query: Query<(Entity, &HousedIn), (With<Person>, Without<Harvests>)>,
    harvester_query: Query<(&Coord, &Range), With<Harvester>>,
    deposit_query: Query<Entity, With<Deposit>>,
    constructions: Res<Constructions>,
    mut commands: Commands,
) {
    for (person, housed_in) in person_query {
        let Ok((coord, range)) = harvester_query.get(housed_in.0) else {
            continue;
        };

        let Some(deposit) = range.iter(coord.0).find_map(|pos| {
            constructions
                .get(&pos)
                .and_then(|d| deposit_query.get(*d).ok())
        }) else {
            continue;
        };

        commands.entity(person).insert(Harvests(deposit));
    }
}

fn harvest_deposit(
    q_people: Query<(&Harvests, &mut HarvestTimer, &HousedIn), With<Person>>,
    mut q_inventories: Query<&mut Inventory>,
    time: Res<Time>,
) {
    for (harvests, mut harvest_timer, housed_in) in q_people {
        if !harvest_timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        let Ok([mut deposit_inventory, mut harvester_inventory]) =
            q_inventories.get_many_mut([harvests.0, housed_in.0])
        else {
            continue;
        };

        let Some((resource_id, quantity)) = deposit_inventory.items.iter_mut().next() else {
            continue;
        };

        if *quantity == 0 {
            continue;
        }

        *quantity -= 1;

        harvester_inventory
            .items
            .entry(*resource_id)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
}

fn cleanup_empty_deposits(
    q_deposits: Query<(Entity, &Inventory, &Coord), With<Deposit>>,
    mut constructions: ResMut<Constructions>,
    mut commands: Commands,
) {
    for (deposit, inventory, coord) in q_deposits {
        if inventory.items.is_empty() {
            constructions.remove(coord);
            commands.entity(deposit).despawn();
        }
    }
}
