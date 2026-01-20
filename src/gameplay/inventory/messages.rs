use bevy::prelude::*;

use super::prelude::*;

/// System set where items are transferred between item slots
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct ItemTransferSystems;

/// Message written when outside systems request an item transfer
/// Handled in batch to produce consistent behavior
#[derive(Message)]
pub struct TransferItems {
    pub from: Entity,
    pub to: Entity,
    pub item: Handle<ItemDef>,
    pub quantity: u32,
}

pub(super) fn transfer_items(
    mut transfer_items: MessageReader<TransferItems>,
    inventory: Query<&Inventory>,
    mut stacks: Query<&mut ItemStack>,
) {
    for TransferItems {
        from,
        to,
        item,
        quantity,
    } in transfer_items.read()
    {
        let Some(from_slot) = inventory
            .iter_descendants(*from)
            .find(|e| stacks.get(*e).is_ok_and(|e| e.item == *item))
        else {
            warn!("Attempted to transfer an item type from an entity that does not own item type");
            continue;
        };

        let Some(to_slot) = inventory
            .iter_descendants(*to)
            .find(|e| stacks.get(*e).is_ok_and(|e| e.item == *item))
        else {
            warn!("Attempted to transfer an item type to an entity that does not own item type");
            continue;
        };

        let Ok([mut from_stack, mut to_stack]) = stacks.get_many_mut([from_slot, to_slot]) else {
            warn!("Attempted to transfer items from non-slot entity: {from} -> {to}");
            continue;
        };

        let actual_quantity = from_stack.quantity.min(*quantity);
        from_stack.quantity = from_stack.quantity.saturating_sub(actual_quantity);
        to_stack.quantity = to_stack.quantity.saturating_add(actual_quantity);
    }
}
