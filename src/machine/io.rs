use bevy::{platform::collections::HashMap, prelude::*};

use crate::machine::{
    Machine,
    work::{BeginWork, WorkCompleted, Working},
};

pub fn plugin(app: &mut App) {
    app.register_type::<TotalInventory>();
    app.register_type::<ResourceOutput>();

    app.init_resource::<TotalInventory>();

    app.add_systems(Update, (begin_work, finish_work));
}

#[derive(Hash, PartialEq, Eq, Reflect, Debug, Clone, Copy)]
pub enum ItemType {
    Coal,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TotalInventory(pub HashMap<ItemType, u32>);

impl TotalInventory {
    fn satisfies(&self, other: &HashMap<ItemType, u32>) -> bool {
        other
            .iter()
            .all(|(k, v)| self.0.get(k).is_some_and(|amount| amount >= v))
    }

    fn append(&mut self, other: &HashMap<ItemType, u32>) {
        for (&k, &v) in other.iter() {
            self.0
                .entry(k)
                .and_modify(|amount| *amount += v)
                .or_insert(v);
        }
    }

    fn subtract(&mut self, other: &HashMap<ItemType, u32>) {
        for (&k, &v) in other.iter() {
            self.0.entry(k).and_modify(|amount| *amount -= v);
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ResourceOutput(pub HashMap<ItemType, u32>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ResourceInput(pub HashMap<ItemType, u32>);

fn begin_work(
    machines: Query<(Entity, &ResourceInput), (With<Machine>, Without<Working>)>,
    mut total_inventory: ResMut<TotalInventory>,
    mut events: EventWriter<BeginWork>,
) {
    for (entity, resource_input) in machines {
        if total_inventory.satisfies(&resource_input.0) {
            total_inventory.subtract(&resource_input.0);
            events.write(BeginWork(entity));
        }
    }
}

fn finish_work(
    mut events: EventReader<WorkCompleted>,
    mut inventory: ResMut<TotalInventory>,
    outputs: Query<&ResourceOutput>,
) {
    for event in events.read() {
        if let Ok(output) = outputs.get(event.0) {
            inventory.append(&output.0);
        }
    }
}
