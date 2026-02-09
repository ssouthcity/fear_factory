use std::{collections::HashSet, time::Duration};

use bevy::{ecs::relationship::OrderedRelationshipSourceCollection, prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};
use rand::seq::{IndexedRandom, IteratorRandom};

use crate::gameplay::{
    inventory::prelude::*,
    people::{Assignees, Person, Porter, profession::ProfessionSystems},
    random::Seed,
    sprite_sort::{YSortSprite, ZIndexSprite},
    world::{
        construction::Constructions,
        tilemap::{CARDINALS, coord::Coord},
    },
};

pub const ARRIVAL_THRESHOLD: f32 = 8.0;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PorterArrival>();
    app.add_message::<PorterLost>();
    app.add_message::<PorterCheckpointReached>();

    app.add_systems(
        FixedUpdate,
        (
            spawn_porter,
            drop_off_items,
            pickup_items,
            returnal,
            (move_towards_target, calculate_next_target).chain(),
            (decrement_ttl, despawn_lost_porters).chain(),
        )
            .in_set(ProfessionSystems),
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
    pub slot: Entity,
}

#[derive(Message, Reflect, Debug)]
pub struct PorterLost(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum PortingState {
    PickingUpItems,
    TransportTo,
    DroppingOffItems,
    Returnal,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Porting {
    pub item: Handle<ItemDef>,
    pub origin: Entity,
    pub slot: Entity,
    pub state: PortingState,

    pub speed: f32,
    pub ttl: Duration,

    pub target: Entity,
    pub backtracking: bool,
    pub visited: HashSet<Entity>,
    pub path: Vec<Entity>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Walkable;

#[derive(Message)]
pub struct PorterCheckpointReached(pub Entity);

fn spawn_porter(
    structure_query: Query<(
        Entity,
        &Transform,
        &Coord,
        &mut PorterCooldown,
        &mut PorterSpawnOutputIndex,
        &Assignees,
    )>,
    constructions: Res<Constructions>,
    person_query: Query<(), (With<Person>, With<Porter>, Without<Porting>)>,
    mut commands: Commands,
    item_defs: Res<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
    inventory: Query<&Inventory>,
    pickup_stacks: Query<&ItemStack, With<Pickup>>,
    walkables: Query<&Walkable>,
    time: Res<Time>,
    mut seed: ResMut<Seed>,
) {
    for (structure, transform, coord, mut timer, mut index, assignees) in structure_query {
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

        let Ok(stack) = pickup_stacks.get(slot) else {
            continue;
        };

        if stack.quantity == 0 {
            continue;
        }

        let Some(item_def) = item_defs.get(&stack.item) else {
            continue;
        };

        let Some(neighbor) = CARDINALS
            .iter()
            .map(|c| coord.0 + c)
            .filter_map(|c| constructions.get(&c))
            .filter(|e| walkables.contains(**e))
            .choose(&mut seed)
        else {
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
                slot,
                state: PortingState::PickingUpItems,

                speed: 64.0,
                ttl: Duration::from_secs(30),

                target: *neighbor,
                backtracking: false,
                visited: HashSet::default(),
                path: Vec::default(),
            },
        ));

        index.0 = (index.0 + 1)
            % inventory
                .iter_descendants(structure)
                .filter(|e| pickup_stacks.contains(*e))
                .collect::<Vec<_>>()
                .len();

        timer.reset();
    }
}

fn pickup_items(
    porters: Query<(Entity, &mut Porting)>,
    inventory: Query<&Inventory>,
    stacks: Query<&ItemStack>,
    mut transfer_items: MessageWriter<TransferItems>,
) {
    for (porter, mut porting) in porters {
        if !matches!(porting.state, PortingState::PickingUpItems) {
            continue;
        }

        let Some(porter_slot) = inventory.iter_descendants(porter).next() else {
            continue;
        };

        if let Ok(stack) = stacks.get(porter_slot)
            && stack.quantity > 0
        {
            porting.state = PortingState::TransportTo;
        } else {
            transfer_items.write(TransferItems {
                from_slot: porting.slot,
                to_slot: porter_slot,
                quantity: 1,
            });
        }
    }
}

fn move_towards_target(
    porters: Query<(Entity, &mut Transform, &mut Sprite, &Porting)>,
    tiles: Query<&Transform, Without<Porting>>,
    mut target_reached: MessageWriter<PorterCheckpointReached>,
    time: Res<Time>,
) {
    for (porter, mut transform, mut sprite, porting) in porters {
        let Ok(target_transform) = tiles.get(porting.target) else {
            continue;
        };

        sprite.flip_x = target_transform.translation.x < transform.translation.x;

        transform.translation = transform.translation.move_towards(
            target_transform.translation,
            porting.speed * time.delta_secs(),
        );

        if transform
            .translation
            .xy()
            .distance(target_transform.translation.xy())
            <= ARRIVAL_THRESHOLD
        {
            target_reached.write(PorterCheckpointReached(porter));
        }
    }
}

fn calculate_next_target(
    mut targets_reached: MessageReader<PorterCheckpointReached>,
    mut porters: Query<&mut Porting>,
    coords: Query<&Coord>,
    walkables: Query<(), With<Walkable>>,
    constructions: Res<Constructions>,
    inventory: Query<&Inventory>,
    drop_off_slots: Query<&DropOff>,
    item_definitions: Res<Assets<ItemDef>>,
    mut porter_arrived: MessageWriter<PorterArrival>,
    mut seed: ResMut<Seed>,
) {
    for PorterCheckpointReached(porter) in targets_reached.read() {
        let Ok(mut porting) = porters.get_mut(*porter) else {
            continue;
        };

        if !matches!(porting.state, PortingState::TransportTo) {
            continue;
        }

        let target = porting.target;

        porting.visited.insert(target);
        porting.path.push(target);

        let Ok(coord) = coords.get(target) else {
            continue;
        };

        let neighbors: Vec<Entity> = CARDINALS
            .iter()
            .map(|c| c + coord.0)
            .filter_map(|c| constructions.get(&c).cloned())
            .collect();

        if let Some(slot) = neighbors
            .iter()
            .filter_map(|construction| {
                inventory.iter_descendants(*construction).find(|slot| {
                    let Ok(drop_off) = drop_off_slots.get(*slot) else {
                        return false;
                    };

                    match drop_off {
                        DropOff::Item(handle) => porting.item == *handle,
                        DropOff::Tag(tag) => item_definitions
                            .get(&porting.item)
                            .is_some_and(|item| item.tags.contains(tag)),
                    }
                })
            })
            .choose(&mut seed)
        {
            porter_arrived.write(PorterArrival {
                porter: *porter,
                slot,
            });
            return;
        }

        let paths: Vec<Entity> = neighbors
            .iter()
            .cloned()
            .filter(|e| {
                walkables.contains(*e) && (porting.backtracking || !porting.visited.contains(e))
            })
            .collect();

        if let Some(t) = paths.choose(&mut seed) {
            porting.target = *t;
            porting.backtracking = false;
        } else if let Some(t) = porting.path.pop() {
            porting.target = t;
            porting.backtracking = true;
        }
    }
}

fn despawn_lost_porters(
    mut porter_losses: MessageReader<PorterLost>,
    mut commands: Commands,
    porters: Query<&Porting>,
    inventory: Query<&Inventory>,
    mut transfer_items: MessageWriter<TransferItems>,
) {
    for PorterLost(entity) in porter_losses.read() {
        if let Ok(porting) = porters.get(*entity)
            && let Some(porter_slot) = inventory.iter_descendants(*entity).next()
        {
            transfer_items.write(TransferItems {
                from_slot: porter_slot,
                to_slot: porting.slot,
                quantity: 1,
            });

            commands.entity(*entity).remove::<(Sprite, Porting)>();
        }
    }
}

fn drop_off_items(
    mut porter_arrivals: MessageReader<PorterArrival>,
    mut porters: Query<(&mut Porting, &mut AseAnimation)>,
    inventory: Query<&Inventory>,
    mut transfer_items: MessageWriter<TransferItems>,
) {
    for PorterArrival { porter, slot } in porter_arrivals.read() {
        let Ok((mut porting, mut animation)) = porters.get_mut(*porter) else {
            continue;
        };

        let Some(porter_slot) = inventory.iter_descendants(*porter).next() else {
            continue;
        };

        transfer_items.write(TransferItems {
            from_slot: porter_slot,
            to_slot: *slot,
            quantity: 1,
        });

        porting.state = PortingState::Returnal;
        animation.animation = Animation::tag("walk");
    }
}

fn returnal(
    mut targets_reached: MessageReader<PorterCheckpointReached>,
    mut porters: Query<&mut Porting>,
    mut commands: Commands,
) {
    for PorterCheckpointReached(porter) in targets_reached.read() {
        let Ok(mut porting) = porters.get_mut(*porter) else {
            continue;
        };

        if !matches!(porting.state, PortingState::Returnal) {
            continue;
        }

        if let Some(path) = porting.path.pop_back() {
            porting.target = path;
            continue;
        };

        commands.entity(*porter).remove::<(Sprite, Porting)>();
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
