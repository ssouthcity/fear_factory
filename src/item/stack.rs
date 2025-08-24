use bevy::prelude::*;

use crate::{
    assets::manifest::{Definition, Id},
    item::Item,
};

#[derive(Debug, Reflect, Clone)]
pub struct Stack {
    pub item_id: Id<Item>,
    pub quantity: u32,
    pub max_quantity: u32,
}

impl Stack {
    pub fn with_quantity(mut self, quantity: u32) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn remaining_space(&self) -> u32 {
        self.max_quantity - self.quantity
    }

    pub fn is_full(&self) -> bool {
        self.quantity >= self.max_quantity
    }
}

impl From<&Definition<Item>> for Stack {
    fn from(value: &Definition<Item>) -> Self {
        Self {
            item_id: value.id.clone(),
            quantity: 0,
            max_quantity: value.stack_size.0,
        }
    }
}
