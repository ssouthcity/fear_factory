use bevy::prelude::*;

use super::prelude::*;

/// System set where items are transferred between item slots
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct ItemTransferSystems;

/// Message written when outside systems request an item transfer
/// Handled in batch to produce consistent behavior
#[derive(Message)]
pub struct TransferItems {
    pub from_slot: Entity,
    pub to_slot: Entity,
    pub quantity: u32,
}

pub(super) fn transfer_items(
    mut transfer_items: MessageReader<TransferItems>,
    mut stacks: Query<&mut ItemStack>,
    mut commands: Commands,
) {
    for TransferItems {
        from_slot,
        to_slot,
        quantity,
    } in transfer_items.read()
    {
        if let Ok([mut from_stack, mut to_stack]) = stacks.get_many_mut([*from_slot, *to_slot]) {
            if from_stack.item != to_stack.item {
                warn!(
                    "Attempted to transfer items between incompatible stacks: {from_slot} -> {to_slot}"
                );
                continue;
            }

            let actual_quantity = from_stack.quantity.min(*quantity);
            from_stack.quantity = from_stack.quantity.saturating_sub(actual_quantity);
            to_stack.quantity = to_stack.quantity.saturating_add(actual_quantity);

            return;
        }

        if let Ok(mut from_stack) = stacks.get_mut(*from_slot) {
            let actual_quantity = from_stack.quantity.min(*quantity);
            from_stack.quantity = from_stack.quantity.saturating_sub(actual_quantity);

            commands.entity(*to_slot).insert(ItemStack {
                item: from_stack.item.clone(),
                quantity: actual_quantity,
                capacity: None,
            });
        }
    }
}
