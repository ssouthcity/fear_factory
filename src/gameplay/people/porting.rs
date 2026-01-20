use std::{collections::HashSet, time::Duration};

use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

use crate::gameplay::{
    FactorySystems,
    inventory::prelude::*,
    people::{Assignees, Person, Porter},
    sprite_sort::{YSortSprite, ZIndexSprite},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PorterArrival>();
    app.add_message::<PorterLost>();

    app.add_systems(
        FixedUpdate,
        (
            spawn_porter,
            drop_of_items,
            (decrement_ttl, despawn_lost_porters).chain(),
        )
            .in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
#[require(PorterSpawnOutputIndex)]
pub struct PorterCooldown(pub Timer);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
struct PorterSpawnOutputIndex(usize);

#[derive(Message, Reflect, Debug)]
pub struct PorterArrival {
    pub porter: Entity,
    pub destination: Entity,
}

#[derive(Message, Reflect, Debug)]
pub struct PorterLost(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Porting {
    pub item: Handle<ItemDef>,
    pub origin: Entity,
    pub slot: Entity,

    pub speed: f32,
    pub ttl: Duration,

    pub target: Entity,
    pub backtracking: bool,
    pub visited: HashSet<Entity>,
    pub path: Vec<Entity>,
}

fn spawn_porter(
    structure_query: Query<(
        Entity,
        &Transform,
        &mut PorterCooldown,
        &mut PorterSpawnOutputIndex,
        &Assignees,
    )>,
    person_query: Query<(), (With<Person>, With<Porter>, Without<Porting>)>,
    mut commands: Commands,
    item_defs: Res<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
    inventory: Query<&Inventory>,
    mut pickup_stacks: Query<&mut ItemStack, With<Pickup>>,
    time: Res<Time>,
) {
    for (structure, transform, mut timer, mut index, assignees) in structure_query {
        if !timer.tick(time.delta()).is_finished() {
            continue;
        }

        let Some(person) = assignees.iter().find(|e| person_query.contains(*e)) else {
            continue;
        };

        let Some(slot) = inventory
            .iter_descendants(structure)
            .find(|e| pickup_stacks.contains(*e))
        else {
            continue;
        };

        let Ok(mut stack) = pickup_stacks.get_mut(slot) else {
            continue;
        };

        if stack.quantity == 0 {
            continue;
        }

        let Some(item_def) = item_defs.get(&stack.item) else {
            continue;
        };

        commands.entity(person).insert((
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
            YSortSprite,
            ZIndexSprite(10),
            Porting {
                item: stack.item.clone(),
                origin: structure,
                slot: slot,

                speed: 64.0,
                ttl: Duration::from_secs(30),

                target: structure,
                backtracking: false,
                visited: HashSet::default(),
                path: Vec::default(),
            },
        ));

        stack.quantity -= 1;

        index.0 = (index.0 + 1)
            % inventory
                .iter_descendants(structure)
                .filter(|e| pickup_stacks.contains(*e))
                .collect::<Vec<_>>()
                .len();

        timer.reset();
    }
}

fn despawn_lost_porters(
    mut porter_losses: MessageReader<PorterLost>,
    mut commands: Commands,
    porters: Query<&Porting>,
    mut slots: Query<&mut ItemStack>,
) {
    for PorterLost(entity) in porter_losses.read() {
        if let Ok(porting) = porters.get(*entity)
            && let Ok(mut stack) = slots.get_mut(porting.slot)
        {
            stack.quantity = stack.quantity.saturating_add(1);
        }

        commands.entity(*entity).remove::<(Sprite, Porting)>();
    }
}

fn drop_of_items(
    mut porter_arrivals: MessageReader<PorterArrival>,
    porters: Query<&Porting>,
    inventory: Query<&Inventory>,
    mut slots: Query<&mut ItemStack, With<DropOff>>,
    mut commands: Commands,
) {
    for PorterArrival {
        porter,
        destination,
    } in porter_arrivals.read()
    {
        commands.entity(*porter).remove::<(Sprite, Porting)>();

        let Ok(porting) = porters.get(*porter) else {
            continue;
        };

        let Some(slot) = inventory
            .iter_descendants(*destination)
            .find(|e| slots.get(*e).is_ok_and(|stack| stack.item == porting.item))
        else {
            continue;
        };

        if let Ok(mut stack) = slots.get_mut(slot) {
            stack.quantity = stack.quantity.saturating_add(1);
        }
    }
}

fn decrement_ttl(
    porters: Query<(Entity, &mut Porting)>,
    time: Res<Time>,
    mut porter_losses: MessageWriter<PorterLost>,
) {
    for (porter, mut porting) in porters {
        porting.ttl = porting.ttl.saturating_sub(time.delta());

        if porting.ttl.is_zero() {
            porter_losses.write(PorterLost(porter));
        }
    }
}
