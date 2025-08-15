use std::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    FactorySystems,
    assets::manifest::{Id, ManifestParam},
    dismantle::QueueDismantle,
    item::{Item, ItemAssets, Stack},
    logistics::{
        ConveyorHoleOf,
        io::{ResourceInputInventory, ResourceOutputInventory},
    },
    sandbox::Sandbox,
    ui::{Interact, Interactable, YSort},
};

/// How much space of the belt should be reserved per item
const CONVEYOR_BELT_TRAY_SIZE: f32 = 16.0;

pub fn plugin(app: &mut App) {
    app.register_type::<QueueConveyorSpawn>();
    app.register_type::<ConveyorBelt>();
    app.register_type::<ConveyorSpeed>();
    app.register_type::<ConveyorLength>();

    app.register_type::<ConveyoredItems>();
    app.register_type::<ConveyoredItem>();
    app.register_type::<ConveyoredItemOf>();

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
#[require(ConveyorLength, ConveyorSpeed, ConveyoredItems)]
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
#[relationship_target(relationship = ConveyoredItemOf, linked_spawn)]
pub struct ConveyoredItems(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConveyoredItem(Id<Item>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ConveyoredItems)]
pub struct ConveyoredItemOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConveyoredItemProgress(f32);

fn build_conveyor_belts(
    mut events: EventReader<QueueConveyorSpawn>,
    transforms: Query<&GlobalTransform>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    sandbox: Single<Entity, With<Sandbox>>,
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
                aseprite: asset_server.load("conveyor.aseprite"),
                animation: Animation::tag("idle"),
            },
            ConveyorBelt(event.0, event.1),
            ConveyorSpeed(60.0),
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

fn place_items_on_belt(
    conveyor_belts: Query<(
        Entity,
        &Transform,
        &ConveyorBelt,
        &ConveyorLength,
        &ConveyoredItems,
        &mut ConveyorPickupTimer,
    )>,
    conveyor_holes: Query<&ConveyorHoleOf>,
    mut outputs: Query<&mut ResourceOutputInventory>,
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    time: Res<Time>,
) {
    for (entity, transform, belt, length, items, mut pickup_timer) in conveyor_belts {
        if !pickup_timer.0.tick(time.delta()).finished() {
            continue;
        }

        let Ok(pickup_point) = conveyor_holes
            .get(belt.0)
            .map(|conveyor_hole_of| conveyor_hole_of.0)
        else {
            continue;
        };

        let Ok(mut output) = outputs.get_mut(pickup_point) else {
            continue;
        };

        if (length.0 / CONVEYOR_BELT_TRAY_SIZE).ceil() as u8 == items.len() as u8 {
            continue;
        }

        if let Ok(item_id) = output.pop() {
            commands
                .spawn((
                    Name::new("Item"),
                    Transform::default()
                        .with_translation(Vec3::new(-length.0 / 2.0, 0.0, 0.0))
                        .with_rotation(transform.rotation.inverse()),
                    item_assets.sprite(item_id.clone()),
                    ConveyoredItemProgress(0.0),
                    ConveyoredItem(item_id.clone()),
                    ConveyoredItemOf(entity),
                    ChildOf(entity),
                    Pickable::default(),
                    Interactable::default(),
                ))
                .observe(|trigger: Trigger<Interact>, mut commands: Commands| {
                    commands.entity(trigger.target()).despawn();
                });
        }
    }
}

fn transfer_belt_contents(
    belt_query: Query<(&ConveyorLength, &ConveyorSpeed, &ConveyoredItems)>,
    mut item_query: Query<(&mut Transform, &mut ConveyoredItemProgress)>,
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
    conveyored_items: Query<(
        Entity,
        &ConveyoredItem,
        &ConveyoredItemProgress,
        &ConveyoredItemOf,
    )>,
    conveyor_holes: Query<&ConveyorHoleOf>,
    mut inputs: Query<&mut ResourceInputInventory>,
    mut commands: Commands,
    item_manifest: ManifestParam<Item>,
) {
    let Some(items) = item_manifest.read() else {
        return;
    };

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

        let Ok(mut input) = inputs.get_mut(dropoff_point) else {
            continue;
        };

        let Some(item_def) = items.get(&item.0) else {
            continue;
        };

        let mut stack = Stack::from(&item_def).with_quantity(1);

        if input.add_stack(&mut stack).is_ok() {
            commands.entity(entity).despawn();
        }
    }
}
