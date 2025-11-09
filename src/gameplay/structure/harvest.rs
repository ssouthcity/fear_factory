use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::gameplay::{
    FactorySystems,
    item::{
        assets::Taxonomy,
        inventory::{SlotOf, Slots},
        stack::Stack,
    },
    recipe::Output,
    structure::range::Range,
    world::{
        construction::{Constructions, StructureConstructed},
        deposit::Deposit,
        tilemap::coord::Coord,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            (add_people_to_harvester, assign_harvester_taxonomy)
                .run_if(on_message::<StructureConstructed>),
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
pub struct Person;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = HousedIn, linked_spawn)]
pub struct Houses(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = Houses)]
pub struct HousedIn(pub Entity);

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

fn add_people_to_harvester(
    mut structures_constructed: MessageReader<StructureConstructed>,
    harvester_query: Query<Entity, With<Harvester>>,
    mut commands: Commands,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        if !harvester_query.contains(*structure) {
            continue;
        }

        for _ in 0..3 {
            commands.spawn((Name::new("Person"), Person, HousedIn(*structure)));
        }
    }
}

fn assign_harvester_taxonomy(
    mut structures_constructed: MessageReader<StructureConstructed>,
    mut harvester_query: Query<(&Coord, &Range, &mut AseAnimation), With<Harvester>>,
    deposit_query: Query<(&Stack, &Taxonomy), With<Deposit>>,
    constructions: Res<Constructions>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        let Ok((coord, range, mut ase_animation)) = harvester_query.get_mut(*structure) else {
            continue;
        };

        let Some((stack, taxonomy)) = range.iter(coord.0).find_map(|pos| {
            constructions
                .get(&pos)
                .and_then(|d| deposit_query.get(*d).ok())
        }) else {
            continue;
        };

        commands.entity(*structure).insert(taxonomy.clone());

        commands.spawn((
            Name::new("Slot"),
            ChildOf(*structure),
            SlotOf(*structure),
            Stack {
                item: stack.item.clone(),
                quantity: 0,
            },
            Output { quantity: 1 },
        ));

        let variant = match taxonomy {
            Taxonomy::Flora => "flora",
            Taxonomy::Fauna => "fauna",
            Taxonomy::Minerale => "minerale",
        };

        ase_animation.aseprite =
            asset_server.load(format!("sprites/structures/harvester_{}.aseprite", variant));
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
    q_harvesters: Query<&Slots, With<Harvester>>,
    mut q_slots: Query<&mut Stack>,
    time: Res<Time>,
) {
    for (harvests, mut harvest_timer, housed_in) in q_people {
        if !harvest_timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        let Some(harvester_slot) = q_harvesters
            .iter_descendants(housed_in.0)
            .find(|slot| q_slots.contains(*slot))
        else {
            continue;
        };

        let Ok([mut deposit_stack, mut harvester_stack]) =
            q_slots.get_many_mut([harvests.0, harvester_slot])
        else {
            continue;
        };

        let to_take = deposit_stack.quantity.min(1);

        deposit_stack.quantity = deposit_stack.quantity.saturating_sub(to_take);
        harvester_stack.quantity = harvester_stack.quantity.saturating_add(to_take);
    }
}

fn cleanup_empty_deposits(
    q_deposits: Query<(Entity, &Stack, &Coord), With<Deposit>>,
    mut constructions: ResMut<Constructions>,
    mut commands: Commands,
) {
    for (deposit, stack, coord) in q_deposits {
        if stack.quantity == 0 {
            constructions.remove(&coord);
            commands.entity(deposit).despawn();
        }
    }
}
