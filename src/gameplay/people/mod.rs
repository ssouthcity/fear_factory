use bevy::prelude::*;

use crate::gameplay::{people::naming::NameManager, world::construction::StructureConstructed};

pub mod foraging;
pub mod naming;
pub mod pathfinding;
pub mod porting;
pub mod profession;

pub use profession::{
    AssignPerson, Assignees, Assignment, Forager, PersonAssignmentChanged, Porter, Profession,
    UnassignPerson,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        foraging::plugin,
        naming::plugin,
        pathfinding::plugin,
        porting::plugin,
        profession::plugin,
    ));

    app.add_systems(
        FixedUpdate,
        add_housed_people_to_new_structures.run_if(on_message::<StructureConstructed>),
    );
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
            let id = commands
                .spawn((Name::new(name_manager.next()), Person))
                .id();

            commands.trigger(AssignPerson {
                person: id,
                structure: *structure,
                profession: Profession::Porter,
            });
        }
    }
}
