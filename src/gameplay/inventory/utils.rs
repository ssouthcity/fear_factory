use std::collections::HashMap;

use bevy::prelude::*;

use super::prelude::*;

pub fn can_afford(
    entity: Entity,
    cost: &HashMap<Handle<ItemDef>, u32>,
    inventory: &Query<&Inventory>,
    // TODO: this function doesn't need mut access to stacks, but spend does.
    // Because both functions are used from the same system, they could only have 1 access level
    stacks: &Query<&mut ItemStack>,
) -> bool {
    if cost.is_empty() {
        return true;
    }

    let mut available: HashMap<AssetId<ItemDef>, u32> = HashMap::new();

    for item_stack in inventory
        .iter_descendants(entity)
        .flat_map(|entity| stacks.get(entity))
    {
        available
            .entry(item_stack.item.id())
            .and_modify(|v| *v += item_stack.quantity)
            .or_insert(item_stack.quantity);
    }

    for (required_item, required_quantity) in cost {
        let available_quantity = available.get(&required_item.id()).copied().unwrap_or(0);

        if available_quantity < *required_quantity {
            return false;
        }
    }

    true
}

pub fn spend(
    entity: Entity,
    cost: &HashMap<Handle<ItemDef>, u32>,
    inventory: &Query<&Inventory>,
    item_stack_query: &mut Query<&mut ItemStack>,
) {
    for (item, quantity) in cost {
        let Some(slot) = inventory.iter_descendants(entity).find(|entity| {
            item_stack_query
                .get(*entity)
                .is_ok_and(|stack| stack.item == *item)
        }) else {
            continue;
        };

        if let Ok(mut stack) = item_stack_query.get_mut(slot) {
            stack.quantity -= quantity;
        }
    }
}
