use bevy::prelude::*;

use super::prelude::*;

pub fn empty_slot(owner: Entity) -> impl Bundle {
    (Name::new("Slot"), InInventory(owner), ChildOf(owner))
}

pub fn item_stack_slot(owner: Entity, item: Handle<ItemDef>, quantity: u32) -> impl Bundle {
    (empty_slot(owner), ItemStack { item, quantity })
}
