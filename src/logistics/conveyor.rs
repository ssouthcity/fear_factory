use std::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    FactorySystems,
    dismantle::QueueDismantle,
    logistics::{
        ItemID, ResourceInput, ResourceOutput,
        io::{ResourceInputInventory, ResourceOutputInventory},
        item::ItemAssets,
    },
    sandbox::Sandbox,
    ui::YSort,
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

    app.add_observer(on_drag_queue_conveyor_belt_spawn);

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
struct QueueConveyorSpawn(Entity, Entity);

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

    let duration = Duration::from_secs_f32(speed.0 / 60.0);

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
pub struct ConveyoredItem(ItemID);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ConveyoredItems)]
pub struct ConveyoredItemOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConveyoredItemProgress(f32);

fn on_drag_queue_conveyor_belt_spawn(
    mut trigger: Trigger<Pointer<DragDrop>>,
    resource_inputs: Query<&ResourceInput>,
    resource_outputs: Query<&ResourceOutput>,
    mut events: EventWriter<QueueConveyorSpawn>,
) {
    let event = trigger.event();

    if event.button != PointerButton::Middle {
        return;
    }

    if !resource_outputs.contains(event.dropped) {
        return;
    }

    if !resource_inputs.contains(event.target) {
        return;
    }

    events.write(QueueConveyorSpawn(event.dropped, event.target));

    trigger.propagate(false);
}

fn build_conveyor_belts(
    mut events: EventReader<QueueConveyorSpawn>,
    transforms: Query<&Transform>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    sandbox: Single<Entity, With<Sandbox>>,
) {
    for event in events.read() {
        let from_transform = transforms.get(event.0).unwrap();
        let to_transform = transforms.get(event.1).unwrap();

        let direction = to_transform.translation - from_transform.translation;

        let rotation = Quat::from_rotation_z(direction.xy().to_angle());

        commands.spawn((
            Name::new("Conveyor Belt"),
            ChildOf(*sandbox),
            Transform::default()
                .with_translation(from_transform.translation + direction * 0.5)
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
        &ConveyorBelt,
        &ConveyorLength,
        &mut ConveyorPickupTimer,
    )>,
    mut outputs: Query<&mut ResourceOutputInventory>,
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    time: Res<Time>,
) {
    for (entity, belt, length, mut pickup_timer) in conveyor_belts {
        if !pickup_timer.0.tick(time.delta()).finished() {
            continue;
        }

        let Ok(mut output) = outputs.get_mut(belt.0) else {
            continue;
        };

        if let Some(item_id) = output.pop() {
            commands
                .spawn((
                    Name::new("Item"),
                    Transform::from_xyz(-length.0 / 2.0, 0.0, 0.0),
                    item_assets.sprite(item_id),
                    ConveyoredItemProgress(0.0),
                    ConveyoredItem(item_id),
                    ConveyoredItemOf(entity),
                    ChildOf(entity),
                    Pickable::default(),
                ))
                .observe(
                    |trigger: Trigger<Pointer<Over>>, mut sprites: Query<&mut Sprite>| {
                        if let Ok(mut sprite) = sprites.get_mut(trigger.target) {
                            sprite.color = Color::hsl(120.0, 1.0, 0.5);
                        }
                    },
                )
                .observe(
                    |mut trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.entity(trigger.target).despawn();
                        trigger.propagate(false);
                    },
                )
                .observe(
                    |trigger: Trigger<Pointer<Out>>, mut sprites: Query<&mut Sprite>| {
                        if let Ok(mut sprite) = sprites.get_mut(trigger.target) {
                            sprite.color = Color::default();
                        }
                    },
                );
        }
    }
}

fn transfer_belt_contents(
    query: Query<(
        &mut Transform,
        &mut ConveyoredItemProgress,
        &ConveyoredItemOf,
    )>,
    belt_speed_query: Query<(&ConveyorLength, &ConveyorSpeed)>,
    time: Res<Time>,
) {
    for (mut transform, mut progress, conveyored_item_of) in query {
        let (length, speed) = belt_speed_query.get(conveyored_item_of.0).unwrap();

        transform.translation.x += speed.0 / 60.0 * time.delta_secs() * CONVEYOR_BELT_TRAY_SIZE;

        progress.0 = (transform.translation.x + length.0 / 2.0) / length.0;
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
    mut inputs: Query<&mut ResourceInputInventory>,
    mut commands: Commands,
) {
    for (entity, item, progress, item_of) in conveyored_items {
        if progress.0 < 1.0 {
            continue;
        }

        let Ok(belt) = conveyor_belts.get(item_of.0) else {
            continue;
        };

        let Ok(mut input) = inputs.get_mut(belt.1) else {
            continue;
        };

        input.0.push(item.0);

        commands.entity(entity).despawn();
    }
}
