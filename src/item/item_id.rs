use bevy::prelude::*;

#[derive(Component, Hash, PartialEq, Eq, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(immutable)]
pub struct ItemID(pub String);

impl std::fmt::Display for ItemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemID({})", self.0)
    }
}
