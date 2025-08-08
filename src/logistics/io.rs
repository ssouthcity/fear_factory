use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    FactorySystems,
    logistics::{ItemID, item::ItemCollection},
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
pub struct ResourceOutput(pub ItemCollection);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
#[require(ResourceInputInventory)]
pub struct ResourceInput(pub ItemCollection);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct ResourceInputInventory(pub ItemCollection);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct ResourceOutputInventory(pub ItemCollection);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct InputFilter(HashSet<ItemID>);

impl InputFilter {
    pub fn with_item(mut self, item_id: ItemID) -> Self {
        self.insert(item_id);
        self
    }
}

fn begin_work(
    machines: Query<
        (Entity, &ResourceInput, &mut ResourceInputInventory),
        (With<Machine>, Without<Working>),
    >,
    mut events: EventWriter<BeginWork>,
) {
    for (entity, resource_input, mut inventory) in machines {
        if inventory.0.subtract(&resource_input.0).is_ok() {
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

        output_inventory.0.add(&output.0);
    }
}
