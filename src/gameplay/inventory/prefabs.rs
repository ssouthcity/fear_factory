use bevy::prelude::*;

use super::prelude::*;

pub fn empty_slot(owner: Entity) -> impl Bundle {
    (Name::new("Slot"), InInventory(owner), ChildOf(owner))
}

pub fn item_stack_slot(owner: Entity, item: Handle<ItemDef>, quantity: u32) -> impl Bundle {
    (
        empty_slot(owner),
        ItemStack {
            item,
            quantity,
            capacity: None,
        },
    )
}

pub fn pickup_slot(owner: Entity, item: Handle<ItemDef>) -> impl Bundle {
    (item_stack_slot(owner, item, 0), Pickup)
}

pub fn dropoff_slot(owner: Entity, item: Handle<ItemDef>) -> impl Bundle {
    (item_stack_slot(owner, item, 0), DropOff)
}

pub fn input_slot(owner: Entity, item: Handle<ItemDef>, requirement: u32) -> impl Bundle {
    (dropoff_slot(owner, item), Input { requirement })
}

pub fn output_slot(owner: Entity, item: Handle<ItemDef>, production: u32) -> impl Bundle {
    (pickup_slot(owner, item), Output { production })
}
