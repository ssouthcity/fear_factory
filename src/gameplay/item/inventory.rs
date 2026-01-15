use std::collections::HashMap;

use bevy::prelude::*;

use crate::gameplay::item::assets::ItemDef;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Inventory {
    pub items: HashMap<AssetId<ItemDef>, u32>,
}
