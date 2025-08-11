use bevy::{platform::collections::HashMap, prelude::*};

use crate::item::ItemID;

pub fn plugin(app: &mut App) {
    app.register_type::<ItemCollection>();
}

#[derive(Reflect, Default, Deref, DerefMut, Clone)]
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

    pub fn pop(&mut self) -> Option<ItemID> {
        let Some(item_id) = self.0.keys().next() else {
            return None;
        };

        let item_id = item_id.clone();

        if let Some(val) = self.0.get_mut(&item_id) {
            *val -= 1;
            if *val == 0 {
                self.0.remove(&item_id);
            }
        }

        Some(item_id)
    }

    pub fn push(&mut self, item_id: ItemID) {
        let entry = self.0.entry(item_id).or_default();
        *entry += 1;
    }
}
