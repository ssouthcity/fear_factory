use bevy::prelude::*;
use serde::Deserialize;

use crate::gameplay::inventory::assets::ItemTag;

use super::prelude::*;

/// Relationship to mark everything in an inventory
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = InInventory)]
pub struct Inventory(Vec<Entity>);

/// Relationship to mark as part of inventory
#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Inventory)]
pub struct InInventory(pub Entity);

/// Represents a stack of items
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ItemStack {
    pub item: Handle<ItemDef>,
    pub quantity: u32,
}

/// Recipe input slot marker
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Input {
    pub requirement: u32,
}

/// Recipe output slot marker
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Output {
    pub production: u32,
}

/// Marks slot for porter drop off
#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum DropOff {
    /// Matches exact item
    Item(Handle<ItemDef>),
    /// Matches item with tag
    Tag(ItemTag),
}

/// Marks slot for porter pickup
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Pickup;

/// Defines which taxonomy an item belongs to
#[derive(Component, Clone, Debug, Deserialize, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum Taxonomy {
    Fauna,
    Flora,
    Minerale,
}

/// Defines which transport method is used for an item
#[derive(Clone, Debug, Deserialize, Reflect)]
pub enum Transport {
    Box,
    Bag,
}
