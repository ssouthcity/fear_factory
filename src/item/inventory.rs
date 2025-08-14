use bevy::prelude::*;

use crate::{
    assets::manifest::Id,
    item::{Item, Stack},
};

#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    #[error("Insufficient items")]
    InsufficientItems,
    #[error("Inventory empty")]
    InventoryEmpty,
    #[error("Inventory full")]
    InventoryFull,
}

#[derive(Debug, Reflect, Default)]
pub struct Inventory {
    slots: Vec<Option<Stack>>,
}

#[allow(dead_code)]
impl Inventory {
    pub fn sized(max_slots: usize) -> Self {
        Self {
            slots: vec![None; max_slots],
        }
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    pub fn len(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity()
    }

    pub fn slots(&self) -> &[Option<Stack>] {
        &self.slots
    }

    pub fn add_stack(&mut self, stack: &mut Stack) -> Result<(), InventoryError> {
        for slot in self.slots.iter_mut() {
            if let Some(slot) = slot {
                if slot.item_id == stack.item_id && stack.quantity > 0 {
                    let add = stack.quantity.min(slot.remaining_space());
                    slot.quantity += add;
                    stack.quantity -= add;
                }
            }
        }

        if stack.quantity > 0 {
            if let Some(slot) = self.slots.iter_mut().find(|s| s.is_none()) {
                *slot = Some(stack.clone());
                stack.quantity = 0;
            } else {
                return Err(InventoryError::InventoryFull);
            }
        }

        Ok(())
    }

    pub fn remove_stack(&mut self, stack: &Stack) -> Result<(), InventoryError> {
        if self.total_quantity_of(&stack.item_id) < stack.quantity {
            return Err(InventoryError::InsufficientItems);
        }

        let mut remaining = stack.quantity;
        for slot in self.slots.iter_mut() {
            if let Some(slot) = slot {
                if slot.item_id == stack.item_id && remaining > 0 {
                    let take = remaining.min(slot.quantity);
                    slot.quantity -= take;
                    remaining -= take;
                }
            }
        }

        Ok(())
    }

    pub fn total_quantity_of(&self, item_id: &Id<Item>) -> u32 {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .filter(|stack| stack.item_id == *item_id)
            .map(|stack| stack.quantity)
            .sum()
    }

    pub fn contains(&self, other: &Inventory) -> bool {
        other
            .slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .all(|stack| self.total_quantity_of(&stack.item_id) >= stack.quantity)
    }

    pub fn add_inventory(&mut self, other: &mut Inventory) -> Result<(), InventoryError> {
        for slot in other.slots.iter_mut() {
            if let Some(stack) = slot {
                self.add_stack(stack)?;
            }
        }

        Ok(())
    }

    pub fn remove_inventory(&mut self, other: &Inventory) -> Result<(), InventoryError> {
        if !self.contains(other) {
            return Err(InventoryError::InsufficientItems);
        }

        for stack in other.slots.iter().filter_map(|slot| slot.as_ref()) {
            self.remove_stack(stack)?;
        }

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Id<Item>, InventoryError> {
        for slot in self.slots.iter_mut() {
            if let Some(stack) = slot
                && stack.quantity > 0
            {
                stack.quantity -= 1;
                return Ok(stack.item_id.clone());
            }
        }

        return Err(InventoryError::InventoryEmpty);
    }

    pub fn push(&mut self, item_id: &Id<Item>) -> Result<(), InventoryError> {
        for slot in self.slots.iter_mut() {
            if let Some(stack) = slot
                && stack.item_id == *item_id
                && !stack.is_full()
            {
                stack.quantity += 1;
                return Ok(());
            }
        }

        return Err(InventoryError::InventoryFull);
    }
}
