use std::collections::HashMap;

use bevy::prelude::*;

// #[derive(Component, Reflect, Debug)]
// #[reflect(Component)]
// #[relationship_target(relationship = StoredBy, linked_spawn)]
// pub struct Storage(Vec<Entity>);

// #[derive(Component, Reflect, Debug)]
// #[reflect(Component)]
// #[relationship(relationship_target = Storage)]
// pub struct StoredBy(pub Entity);

#[derive(Component, Reflect, Debug, Hash, PartialEq, Eq, Clone)]
#[reflect(Component)]
pub struct ResourceID(pub String);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ResourceStorage {
    pub resources: HashMap<ResourceID, u32>,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct InputStorage {
    pub resources: HashMap<ResourceID, u32>,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct OutputStorage {
    pub resources: HashMap<ResourceID, u32>,
}
