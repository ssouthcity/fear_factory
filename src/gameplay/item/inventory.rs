use std::collections::HashMap;

use bevy::prelude::*;

use crate::gameplay::item::assets::ItemDef;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Inventory {
    pub items: HashMap<AssetId<ItemDef>, u32>,
}

impl Inventory {
    pub fn with_single(item_id: AssetId<ItemDef>, amount: u32) -> Self {
        Self {
            items: HashMap::from([(item_id, amount)]),
        }
    }
}
