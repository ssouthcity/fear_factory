use bevy::prelude::*;

use crate::gameplay::{
    inventory::prelude::*, people::naming::NameManager, world::construction::StructureConstructed,
};

pub mod foraging;
pub mod naming;
pub mod porting;
pub mod profession;

#[allow(unused_imports)]
pub use profession::{
    AssignPerson, Assignees, Assignment, Forager, PersonAssignmentChanged, Porter, Profession,
    UnassignPerson,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        foraging::plugin,
        naming::plugin,
        porting::plugin,
        profession::plugin,
    ));

    app.add_systems(
        FixedUpdate,
        add_housed_people_to_new_structures.run_if(on_message::<StructureConstructed>),
    );

    app.add_observer(on_person_add);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Person;

fn add_housed_people_to_new_structures(
    mut structures_constructed: MessageReader<StructureConstructed>,
    mut commands: Commands,
    mut name_manager: ResMut<NameManager>,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        for _ in 0..3 {
            let id = commands.spawn(person(&mut name_manager)).id();

            commands.trigger(AssignPerson {
                person: id,
                structure: *structure,
                profession: Profession::Porter,
            });
        }
    }
}

pub fn person(name_manager: &mut NameManager) -> impl Bundle {
    (Name::new(name_manager.next()), Person)
}

fn on_person_add(add: On<Add, Person>, mut commands: Commands) {
    commands.spawn(empty_slot(add.entity));
}
