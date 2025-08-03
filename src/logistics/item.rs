use std::collections::HashMap;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AseSlice, Aseprite};

pub fn plugin(app: &mut App) {
    app.register_type::<ItemID>();
    app.register_type::<ItemCollection>();

    app.register_type::<ItemAssets>();
    app.init_resource::<ItemAssets>();
    app.add_systems(Startup, load_item_assets);
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ItemAssets {
    aseprite: Handle<Aseprite>,
}

impl ItemAssets {
    pub fn sprite(&self, item: ItemID) -> impl Bundle {
        (
            Sprite::sized(Vec2::splat(16.0)),
            AseSlice {
                aseprite: self.aseprite.clone(),
                name: match item {
                    ItemID::Coal => "coal".to_string(),
                    ItemID::IronOre => "iron ore".to_string(),
                    ItemID::IronIngot => "iron ingot".to_string(),
                },
            },
        )
    }
}

fn load_item_assets(mut item_assets: ResMut<ItemAssets>, asset_server: Res<AssetServer>) {
    item_assets.aseprite = asset_server.load("items.aseprite");
}

#[derive(Hash, PartialEq, Eq, Reflect, Debug, Clone, Copy)]
pub enum ItemID {
    Coal,
    IronOre,
    IronIngot,
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
