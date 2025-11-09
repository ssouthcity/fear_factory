use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = SlotOf, linked_spawn)]
pub struct Slots(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = Slots)]
pub struct SlotOf(pub Entity);
