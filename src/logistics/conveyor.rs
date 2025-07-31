use std::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    FactorySystems,
    logistics::{
        ItemID, ResourceInput, ResourceOutput,
        io::{ResourceInputInventory, ResourceOutputInventory},
    },
};

pub fn plugin(app: &mut App) {
    app.register_type::<QueueConveyorSpawn>();
    app.register_type::<ConveyorBelt>();

    app.add_event::<QueueConveyorSpawn>();

    app.add_observer(on_drag_queue_conveyor_belt_spawn);

    app.add_systems(
        Update,
        (
            build_conveyor_belts.in_set(FactorySystems::Build),
            (
                place_items_on_belt,
                transfer_belt_contents,
                receive_items_from_belt,
            )
                .chain()
                .in_set(FactorySystems::Logistics),
            adjust_item_sprites.in_set(FactorySystems::UI),
            // draw_conveyor_belts.in_set(FactorySystems::UI),
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
    trigger: Trigger<Pointer<DragDrop>>,
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
}

fn build_conveyor_belts(
    mut events: EventReader<QueueConveyorSpawn>,
    transforms: Query<&Transform>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in events.read() {
        let from_transform = transforms.get(event.0).unwrap();
        let to_transform = transforms.get(event.1).unwrap();

        let direction = to_transform.translation - from_transform.translation;

        let rotation = Quat::from_rotation_z(direction.xy().to_angle());

        commands.spawn((
            Name::new("Conveyor Belt"),
            Transform::default()
                .with_translation(from_transform.translation.with_z(0.5) + direction * 0.5)
                .with_rotation(rotation),
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
            ConveyorSpeed(40.0),
            ConveyorLength(direction.length()),
        ));
    }
}

fn place_items_on_belt(
    conveyor_belts: Query<(Entity, &ConveyorBelt, &mut ConveyorPickupTimer)>,
    mut outputs: Query<&mut ResourceOutputInventory>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, belt, mut pickup_timer) in conveyor_belts {
        if !pickup_timer.0.tick(time.delta()).finished() {
            continue;
        }

        let Ok(mut output) = outputs.get_mut(belt.0) else {
            continue;
        };

        if let Some(item_id) = output.pop() {
            commands.spawn((
                Name::new("Item"),
                Sprite::from_color(Color::WHITE, Vec2::splat(8.0)),
                ConveyoredItemProgress(0.0),
                ConveyoredItem(item_id),
                ConveyoredItemOf(entity),
                ChildOf(entity),
            ));
        }
    }
}

fn transfer_belt_contents(
    conveyored_item_progresses: Query<&mut ConveyoredItemProgress>,
    time: Res<Time>,
) {
    for mut progress in conveyored_item_progresses {
        progress.0 += time.delta_secs() * 2.0;
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

fn adjust_item_sprites(
    conveyored_items: Query<(&ConveyoredItemProgress, &ConveyoredItemOf, &mut Transform)>,
    conveyor_lengths: Query<&ConveyorLength>,
) {
    for (progress, conveyored_item_of, mut transform) in conveyored_items {
        let length = conveyor_lengths.get(conveyored_item_of.0).unwrap();
        transform.translation.x = (-length.0 * 0.5) + length.0 * progress.0;
    }
}

// fn draw_conveyor_belts(
//     conveyor_belts: Query<&ConveyorBelt>,
//     transforms: Query<&Transform>,
//     mut gizmos: Gizmos,
// ) {
//     for belt in conveyor_belts {
//         let from_position = transforms.get(belt.0).unwrap().translation.truncate();
//         let to_position = transforms.get(belt.1).unwrap().translation.truncate();

//         gizmos.line_2d(from_position, to_position, Color::hsl(180.0, 1.0, 0.5));
//     }
// }
