use bevy::prelude::*;

use crate::{
    assets::{Definition, Id},
    item::Item,
};

#[derive(Debug, Clone)]
pub struct Stack {
    pub item_id: Id<Item>,
    pub quantity: u32,
    pub max_quantity: u32,
}

impl Stack {
    pub fn space_left(&self) -> u32 {
        self.max_quantity - self.quantity
    }

    pub fn is_full(&self) -> bool {
        self.quantity >= self.max_quantity
    }
}

impl<'a> From<&Definition<'a, Item>> for Stack {
    fn from(value: &Definition<'a, Item>) -> Self {
        Self {
            item_id: value.id.clone(),
            quantity: 0,
            max_quantity: value.definition.stack_size,
        }
    }
}

#[derive(Debug, Default)]
pub struct Inventory {
    pub slots: Vec<Stack>,
}

impl Inventory {
    pub fn add_stack(&mut self, stack: &mut Stack) {
        let mut existing_stacks = self.slots.iter_mut().filter(|s| s.item_id == stack.item_id);

        while stack.quantity > 0 {
            let Some(current) = existing_stacks.next() else {
                break;
            };

            let diff = stack.quantity.min(current.space_left());
            current.quantity += diff;
            stack.quantity -= diff;
        }

        // TODO: might add a max_slot check here to ensure that a new slot can be added
        if stack.quantity > 0 {
            self.slots.push(stack.clone());
        }
    }

    pub fn stacks_of(&self, item_id: &Id<Item>) -> Vec<&Stack> {
        self.slots
            .iter()
            .filter(|stack| stack.item_id == *item_id)
            .collect()
    }

    pub fn total_quantity_of(&self, item_id: &Id<Item>) -> u32 {
        self.slots
            .iter()
            .filter(|stack| stack.item_id == *item_id)
            .map(|stack| stack.quantity)
            .sum()
    }

    pub fn contains(&self, other: &Inventory) -> bool {
        other
            .slots
            .iter()
            .all(|stack| self.total_quantity_of(&stack.item_id) >= stack.quantity)
    }
}
