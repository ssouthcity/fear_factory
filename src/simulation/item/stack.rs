use bevy::prelude::*;

#[derive(Debug, Reflect, Clone)]
pub struct Stack {
    pub item_id: String,
    pub quantity: u32,
    pub max_quantity: u32,
}

impl Stack {
    pub fn remaining_space(&self) -> u32 {
        self.max_quantity - self.quantity
    }

    pub fn is_full(&self) -> bool {
        self.quantity >= self.max_quantity
    }
}
