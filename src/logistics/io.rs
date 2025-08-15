use bevy::prelude::*;

use crate::{
    FactorySystems,
    item::{Inventory, Stack},
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
        (begin_work, move_output_to_output_inventory)
            .chain()
            .in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
#[require(ResourceInput, ResourceOutputInventory)]
pub struct ResourceOutput(pub Vec<Stack>);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
#[require(ResourceInputInventory)]
pub struct ResourceInput(pub Vec<Stack>);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct ResourceInputInventory(pub Inventory);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct ResourceOutputInventory(pub Inventory);

fn begin_work(
    machines: Query<
        (Entity, &ResourceInput, &mut ResourceInputInventory),
        (With<Machine>, Without<Working>),
    >,
    mut events: EventWriter<BeginWork>,
) {
    for (entity, resource_input, mut inventory) in machines {
        let has_enough_items = resource_input
            .0
            .iter()
            .all(|stack| inventory.total_quantity_of(&stack.item_id) >= stack.quantity);

        if !has_enough_items {
            return;
        }

        for stack in resource_input.0.iter() {
            let _ = inventory.remove_stack(stack);
        }

        events.write(BeginWork(entity));
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

        for stack in output.0.clone().iter_mut() {
            let _ = output_inventory.0.add_stack(stack);
        }
    }
}
