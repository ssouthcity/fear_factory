use std::collections::HashMap;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<ItemID>();
    app.register_type::<ItemCollection>();
}

#[derive(Hash, PartialEq, Eq, Reflect, Debug, Clone, Copy)]
pub enum ItemID {
    Coal,
}

#[derive(Reflect, Default)]
pub struct ItemCollection(HashMap<ItemID, u32>);

impl ItemCollection {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn with_item(mut self, item: ItemID, quantity: u32) -> Self {
        self.0.insert(item, quantity);
        self
    }

    pub fn contains(&self, other: &Self) -> bool {
        other
            .0
            .iter()
            .all(|(item_id, quantity)| self.0.get(item_id).is_some_and(|held| held >= quantity))
    }

    pub fn add(&mut self, other: &Self) {
        for (item_id, quantity) in other.0.iter() {
            let entry = self.0.entry(*item_id).or_default();
            *entry += quantity;
        }
    }

    pub fn subtract(&mut self, other: &Self) -> Result {
        if !self.contains(other) {
            return Err("Cannot subtract item collection as it would underflow".into());
        }

        for (item_id, quantity) in other.0.iter() {
            let entry = self.0.entry(*item_id).or_default();
            *entry -= quantity;
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
