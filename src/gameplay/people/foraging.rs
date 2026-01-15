use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    item::inventory::Inventory,
    people::{Assignment, Forager, Person},
    structure::{
        deposit::{Deposit, DepositDef},
        foragers_outpost::ForagersOutpost,
        range::Range,
    },
    world::{construction::Constructions, tilemap::coord::Coord},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            assign_deposit_to_forager,
            forage_deposit,
            cleanup_empty_deposits,
        )
            .chain()
            .in_set(FactorySystems::Forage),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = Forages)]
pub struct ForagedBy(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = ForagedBy)]
#[require(ForagingTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))]
pub struct Forages(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ForagingTimer(pub Timer);

fn assign_deposit_to_forager(
    foragers: Query<(Entity, &Assignment), (With<Person>, With<Forager>, Without<Forages>)>,
    foragers_outposts: Query<(&Coord, &Range), With<ForagersOutpost>>,
    deposit_query: Query<Entity, With<Deposit>>,
    constructions: Res<Constructions>,
    mut commands: Commands,
) {
    for (person, assignment) in foragers {
        let Ok((coord, range)) = foragers_outposts.get(assignment.structure) else {
            continue;
        };

        let Some(deposit) = range.iter(coord.0).find_map(|pos| {
            constructions
                .get(&pos)
                .and_then(|d| deposit_query.get(*d).ok())
        }) else {
            continue;
        };

        commands.entity(person).insert(Forages(deposit));
    }
}

fn forage_deposit(
    foragers: Query<(&Forages, &mut ForagingTimer, &Assignment), With<Person>>,
    mut foragers_outpost: Query<&mut Inventory>,
    mut deposits: Query<&mut Deposit>,
    deposit_definitions: Res<Assets<DepositDef>>,
    time: Res<Time>,
) {
    for (forages, mut foraging_timer, assignment) in foragers {
        if !foraging_timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        let Ok(mut foragers_outpost_inventory) = foragers_outpost.get_mut(assignment.structure)
        else {
            continue;
        };

        let Ok(mut deposit) = deposits.get_mut(forages.0) else {
            continue;
        };

        let Some(deposit_def) = deposit_definitions.get(&deposit.handle) else {
            continue;
        };

        if deposit.quantity == 0 {
            continue;
        }

        deposit.quantity -= 1;

        foragers_outpost_inventory
            .items
            .entry(deposit_def.item_id)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
}

fn cleanup_empty_deposits(
    deposits: Query<(Entity, &Deposit, &Coord)>,
    mut constructions: ResMut<Constructions>,
    mut commands: Commands,
) {
    for (entity, deposit, coord) in deposits {
        if deposit.quantity == 0 {
            constructions.remove(coord);
            commands.entity(entity).despawn();
        }
    }
}
