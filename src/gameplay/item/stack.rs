use bevy::prelude::*;

use crate::gameplay::item::assets::ItemDef;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Stack {
    pub item: Handle<ItemDef>,
    pub quantity: u32,
}

impl Stack {
    pub fn empty(item: Handle<ItemDef>) -> Stack {
        let quantity = 0;
        Stack { item, quantity }
    }
}
