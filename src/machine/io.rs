use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    FactorySystems,
    machine::{
        Machine,
        work::{BeginWork, WorkCompleted, Working},
    },
};

pub fn plugin(app: &mut App) {
    app.register_type::<ResourceInput>();
    app.register_type::<ResourceInputInventory>();
    app.register_type::<ResourceOutput>();
    app.register_type::<ResourceOutputInventory>();

    app.add_systems(
        Update,
        (
            begin_work,
            move_output_to_output_inventory,
            transfer_belt_contents,
        )
            .chain()
            .in_set(FactorySystems::Logistics),
    );

    app.register_type::<ConveyorBelt>()
        .add_observer(on_drag_spawn_conveyor_belt)
        .add_systems(Update, draw_conveyor_belts.in_set(FactorySystems::UI));
}

#[derive(Hash, PartialEq, Eq, Reflect, Debug, Clone, Copy)]
pub enum ItemType {
    Coal,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ResourceOutput(pub HashMap<ItemType, u32>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ResourceInput(pub HashMap<ItemType, u32>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ResourceInputInventory(pub HashMap<ItemType, u32>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ResourceOutputInventory(pub HashMap<ItemType, u32>);

fn begin_work(
    machines: Query<
        (Entity, &ResourceInput, &mut ResourceInputInventory),
        (With<Machine>, Without<Working>),
    >,
    mut events: EventWriter<BeginWork>,
) {
    for (entity, resource_input, mut resource_input_inventory) in machines {
        if resource_input.0.iter().all(|(item, quantity)| {
            resource_input_inventory
                .0
                .get(item)
                .is_some_and(|v| v >= quantity)
        }) {
            for (item, quantity) in resource_input.0.iter() {
                resource_input_inventory
                    .0
                    .entry(*item)
                    .and_modify(|v| *v -= quantity);
            }

            events.write(BeginWork(entity));
        }
    }
}

fn move_output_to_output_inventory(
    mut events: EventReader<WorkCompleted>,
    mut outputs: Query<(&ResourceOutput, &mut ResourceOutputInventory)>,
) {
    for event in events.read() {
        let Ok((output, mut output_inventory)) = outputs.get_mut(event.0) else {
            continue;
        };

        for (item_id, quantity) in output.0.iter() {
            output_inventory
                .0
                .entry(*item_id)
                .and_modify(|v| *v += quantity)
                .or_insert(*quantity);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConveyorBelt(Entity, Entity);

fn on_drag_spawn_conveyor_belt(
    trigger: Trigger<Pointer<DragDrop>>,
    resource_inputs: Query<&ResourceInput>,
    resource_outputs: Query<&ResourceOutput>,
    mut commands: Commands,
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

    commands.spawn(ConveyorBelt(event.dropped, event.target));
}

fn draw_conveyor_belts(
    conveyor_belts: Query<&ConveyorBelt>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for belt in conveyor_belts {
        let from_position = transforms.get(belt.0).unwrap().translation.truncate();
        let to_position = transforms.get(belt.1).unwrap().translation.truncate();

        gizmos.line_2d(from_position, to_position, Color::hsl(180.0, 1.0, 0.5));
    }
}

fn transfer_belt_contents(
    conveyor_belts: Query<&ConveyorBelt>,
    mut outputs: Query<&mut ResourceOutputInventory>,
    mut inputs: Query<&mut ResourceInputInventory>,
) {
    for belt in conveyor_belts {
        let Ok(mut output) = outputs.get_mut(belt.0) else {
            continue;
        };

        let Ok(mut input) = inputs.get_mut(belt.1) else {
            continue;
        };

        for (item_id, quantity) in output.0.iter() {
            input
                .0
                .entry(*item_id)
                .and_modify(|v| *v += quantity)
                .or_insert(*quantity);
        }

        output.0.clear();
    }
}
