use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    inventory::prelude::*,
    people::{Assignment, Forager, Person},
    structure::{deposit::Deposit, foragers_outpost::ForagersOutpost, range::Range},
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
    inventory: Query<&Inventory>,
    stacks: Query<&ItemStack>,
    time: Res<Time>,
    mut transfer_items: MessageWriter<TransferItems>,
) {
    for (forages, mut foraging_timer, assignment) in foragers {
        if !foraging_timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        let Some(stack) = inventory
            .iter_descendants(forages.0)
            .find_map(|e| stacks.get(e).ok())
        else {
            continue;
        };

        transfer_items.write(TransferItems {
            from: forages.0,
            to: assignment.structure,
            item: stack.item.clone(),
            quantity: 1,
        });
    }
}

fn cleanup_empty_deposits(
    deposits: Query<(Entity, &Coord), With<Deposit>>,
    inventory: Query<&Inventory>,
    slots: Query<&ItemStack>,
    mut constructions: ResMut<Constructions>,
    mut commands: Commands,
) {
    for (entity, coord) in deposits {
        let deposit_emptied = inventory
            .iter_descendants(entity)
            .all(|slot| slots.get(slot).is_ok_and(|stack| stack.quantity == 0));

        if deposit_emptied {
            constructions.remove(coord);
            commands.entity(entity).despawn();
        }
    }
}
