use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = StoredBy, linked_spawn)]
pub struct Storage(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = Storage)]
pub struct StoredBy(pub Entity);
