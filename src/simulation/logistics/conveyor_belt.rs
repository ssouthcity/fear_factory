use std::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    simulation::{
        FactorySystems,
        dismantle::QueueDismantle,
        item::{Full, Item, ItemAssets, ItemDef, Quantity},
        logistics::{ConveyorHoleOf, LogisticAssets},
        recipe::{Inputs, Outputs},
        world::Terrain,
    },
    ui::{Interact, Interactable, YSort},
};

/// How much space of the belt should be reserved per item
const CONVEYOR_BELT_TRAY_SIZE: f32 = 16.0;

pub fn plugin(app: &mut App) {
    app.register_type::<QueueConveyorSpawn>();
    app.register_type::<ConveyorBelt>();
    app.register_type::<ConveyorSpeed>();
    app.register_type::<ConveyorLength>();

    app.register_type::<Transports>();
    app.register_type::<TransportedBy>();

    app.add_event::<QueueConveyorSpawn>();

    app.add_systems(
        Update,
        (
            build_conveyor_belts.in_set(FactorySystems::Build),
            garbage_clean_conveyor_belts.in_set(FactorySystems::Dismantle),
            (
                place_items_on_belt,
                transfer_belt_contents,
                receive_items_from_belt,
            )
                .chain()
                .in_set(FactorySystems::Logistics),
        ),
    );
}

#[derive(Event, Reflect)]
pub struct QueueConveyorSpawn(pub Entity, pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(ConveyorLength, ConveyorSpeed, Transports)]
pub struct ConveyorBelt(Entity, Entity);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ConveyorLength(f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_insert = insert_pickup_timer)]
pub struct ConveyorSpeed(f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ConveyorCapacity(f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ConveyorPickupTimer(Timer);

fn insert_pickup_timer(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let speed = world.get::<ConveyorSpeed>(entity).unwrap();

    let duration = Duration::from_secs_f32(60.0 / speed.0);

    world
        .commands()
        .entity(entity)
        .insert(ConveyorPickupTimer(Timer::new(
            duration,
            TimerMode::Repeating,
        )));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = TransportedBy, linked_spawn)]
pub struct Transports(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Transports)]
#[require(TransportProgress)]
pub struct TransportedBy(pub Entity);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TransportProgress(f32);

fn build_conveyor_belts(
    mut events: EventReader<QueueConveyorSpawn>,
    transforms: Query<&GlobalTransform>,
    logistic_assets: Res<LogisticAssets>,
    mut commands: Commands,
    sandbox: Single<Entity, With<Terrain>>,
) {
    for event in events.read() {
        let from_transform = transforms.get(event.0).unwrap();
        let to_transform = transforms.get(event.1).unwrap();

        let direction = to_transform.translation() - from_transform.translation();

        let rotation = Quat::from_rotation_z(direction.xy().to_angle());

        commands.spawn((
            Name::new("Conveyor Belt"),
            ChildOf(*sandbox),
            Transform::default()
                .with_translation(from_transform.translation() + direction * 0.5)
                .with_rotation(rotation),
            YSort(0.5),
            Sprite {
                custom_size: Some(Vec2::new(direction.length(), 32.0)),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0,
                },
                ..default()
            },
            AseAnimation {
                aseprite: logistic_assets.conveyor_belt.clone(),
                animation: Animation::tag("idle"),
            },
            ConveyorBelt(event.0, event.1),
            ConveyorSpeed(100.0),
            ConveyorCapacity(direction.length() / CONVEYOR_BELT_TRAY_SIZE),
            ConveyorLength(direction.length()),
        ));
    }
}

fn garbage_clean_conveyor_belts(
    mut events: EventReader<QueueDismantle>,
    conveyor_belts: Query<(Entity, &ConveyorBelt)>,
    mut commands: Commands,
) {
    for event in events.read() {
        for (entity, ConveyorBelt(from, to)) in conveyor_belts {
            if *from == event.0 || *to == event.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn place_items_on_belt(
    conveyor_belts: Query<(
        Entity,
        &Transform,
        &ConveyorBelt,
        &ConveyorLength,
        &Transports,
        &mut ConveyorPickupTimer,
    )>,
    conveyor_holes: Query<&ConveyorHoleOf>,
    outputs_query: Query<&Outputs>,
    output_query: Query<(&Item, &Quantity)>,
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    items: Res<Assets<ItemDef>>,
    time: Res<Time>,
) {
    for (entity, transform, belt, length, conveyored_items, mut pickup_timer) in conveyor_belts {
        if !pickup_timer.0.tick(time.delta()).finished() {
            continue;
        }

        if (length.0 / CONVEYOR_BELT_TRAY_SIZE).ceil() as u8 == conveyored_items.len() as u8 {
            continue;
        }

        let Ok(pickup_point) = conveyor_holes
            .get(belt.0)
            .map(|conveyor_hole_of| conveyor_hole_of.0)
        else {
            continue;
        };

        let Ok(outputs) = outputs_query.get(pickup_point) else {
            continue;
        };

        let Some(item) = outputs
            .iter()
            .flat_map(|e| output_query.get(e))
            .find(|(_, q)| q.0 > 0)
            .map(|(i, _)| i)
        else {
            continue;
        };

        let item_id = items
            .get(&item.0)
            .map(|def| def.id.to_owned())
            .unwrap_or("unknown".to_string());

        commands
            .spawn((
                Name::new("Item"),
                Transform::default()
                    .with_translation(Vec3::new(-length.0 / 2.0, 0.0, 0.0))
                    .with_rotation(transform.rotation.inverse()),
                Sprite::sized(Vec2::splat(16.0)),
                AseSlice {
                    aseprite: item_assets.aseprite.clone(),
                    name: item_id,
                },
                Item(item.0.clone()),
                TransportedBy(entity),
                ChildOf(entity),
                Interactable,
            ))
            .observe(|trigger: Trigger<Interact>, mut commands: Commands| {
                commands.entity(trigger.target()).despawn();
            });
    }
}

fn transfer_belt_contents(
    belt_query: Query<(&ConveyorLength, &ConveyorSpeed, &Transports)>,
    mut item_query: Query<(&mut Transform, &mut TransportProgress)>,
    time: Res<Time>,
) {
    for (length, speed, conveyored_items) in belt_query {
        for (index, item) in conveyored_items.iter().enumerate() {
            let (mut transform, mut progress) = item_query.get_mut(item).unwrap();

            transform.translation.x += speed.0 / 60.0 * CONVEYOR_BELT_TRAY_SIZE * time.delta_secs();

            transform.translation.x = transform.translation.x.clamp(
                -length.0 / 2.0,
                length.0 / 2.0 - CONVEYOR_BELT_TRAY_SIZE * index as f32,
            );

            progress.0 = (transform.translation.x + length.0 / 2.0) / length.0;
        }
    }
}

fn receive_items_from_belt(
    conveyor_belts: Query<&ConveyorBelt>,
    conveyored_items: Query<(Entity, &Item, &TransportProgress, &TransportedBy)>,
    conveyor_holes: Query<&ConveyorHoleOf>,
    recipe_inputs: Query<&Inputs>,
    mut inputs: Query<(&Item, &mut Quantity), Without<Full>>,
    mut commands: Commands,
) {
    for (entity, item, progress, item_of) in conveyored_items {
        if progress.0 < 1.0 {
            continue;
        }

        let Ok(belt) = conveyor_belts.get(item_of.0) else {
            continue;
        };

        let Ok(dropoff_point) = conveyor_holes
            .get(belt.1)
            .map(|conveyor_hole_of| conveyor_hole_of.0)
        else {
            continue;
        };

        let Ok(input_entities) = recipe_inputs.get(dropoff_point) else {
            continue;
        };

        for input_entity in input_entities.iter() {
            let Ok((recipe_item, mut quantity)) = inputs.get_mut(input_entity) else {
                continue;
            };

            if recipe_item.0 == item.0 {
                quantity.0 += 1;
                commands.entity(entity).despawn();
                break;
            }
        }
    }
}
