use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PersonAssignmentChanged>();

    app.add_observer(on_assign_person);
    app.add_observer(on_unassign_person);
}

/// Event to trigger when changing a person's assignment
#[derive(Event)]
pub struct AssignPerson {
    pub person: Entity,
    pub structure: Entity,
    pub profession: Profession,
}

/// Event to trigger when changing a person's assignment
#[derive(Event)]
pub struct UnassignPerson {
    pub person: Entity,
}

/// Message emitted when a persons assignment has changed
#[derive(Message)]
pub struct PersonAssignmentChanged {
    #[allow(dead_code)]
    pub person: Entity,
}

/// Tracks which people are assigned to a structure
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = Assignment)]
pub struct Assignees(Vec<Entity>);

/// Person's assignment to a structure
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = Assignees)]
pub struct Assignment {
    #[relationship]
    pub structure: Entity,
    pub profession: Profession,
}

/// Valid professions for a person
#[derive(Reflect, Debug, Default, Clone, Copy)]
pub enum Profession {
    #[default]
    Forager,
    Porter,
}

/// Marker component for foragers
#[derive(Component)]
pub struct Forager;

/// Marker component for porters
#[derive(Component)]
pub struct Porter;

fn on_assign_person(
    assign_person: On<AssignPerson>,
    mut commands: Commands,
    mut events: MessageWriter<PersonAssignmentChanged>,
) {
    let mut entity = commands.entity(assign_person.person);

    entity.remove::<(Forager, Porter)>();

    entity.insert(Assignment {
        structure: assign_person.structure,
        profession: assign_person.profession,
    });

    match assign_person.profession {
        Profession::Forager => entity.insert(Forager),
        Profession::Porter => entity.insert(Porter),
    };

    events.write(PersonAssignmentChanged {
        person: entity.id(),
    });
}

fn on_unassign_person(
    unassign_person: On<UnassignPerson>,
    mut commands: Commands,
    mut events: MessageWriter<PersonAssignmentChanged>,
) {
    let mut entity = commands.entity(unassign_person.person);

    entity.remove::<(Forager, Porter)>();

    events.write(PersonAssignmentChanged {
        person: entity.id(),
    });
}
