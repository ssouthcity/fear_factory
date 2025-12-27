use bevy::prelude::*;

use crate::gameplay::world::construction::StructureConstructed;

pub mod naming;
pub mod pathfinding;
pub mod porting;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((naming::plugin, pathfinding::plugin, porting::plugin));

    app.add_systems(
        FixedUpdate,
        add_housed_people_to_new_structures.run_if(on_message::<StructureConstructed>),
    );
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Person;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = HousedIn, linked_spawn)]
pub struct Houses(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = Houses)]
pub struct HousedIn(pub Entity);

fn add_housed_people_to_new_structures(
    mut structures_constructed: MessageReader<StructureConstructed>,
    mut commands: Commands,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        for _ in 0..3 {
            commands.spawn((Name::new("Person"), Person, HousedIn(*structure)));
        }
    }
}
