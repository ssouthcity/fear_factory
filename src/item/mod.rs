use bevy::prelude::*;

mod assets;
mod collection;
mod inventory;
mod item_id;
mod recipes;

pub use assets::ItemAssets;
pub use collection::ItemCollection;
pub use item_id::ItemID;
pub use recipes::{Recipe, SelectRecipe, SelectedRecipe};
use serde::Deserialize;

use crate::{
    assets::{ManifestParam, ManifestPlugin},
    item::inventory::{Inventory, Stack},
};

pub fn plugin(app: &mut App) {
    app.add_plugins((ManifestPlugin::<Item>::new("manifest/items.toml"),));

    app.add_plugins((assets::plugin, collection::plugin, recipes::plugin));

    app.add_systems(Update, test);
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Item {
    name: String,
    stack_size: u32,
}

fn test(manifest: ManifestParam<Item>) {
    let Some(manifest) = manifest.get() else {
        return;
    };

    let Some(coal_definition) = manifest.get(&"coal".into()) else {
        return;
    };

    let mut coal_one = Stack::from(&coal_definition);
    coal_one.quantity = 73;

    let mut coal_two = Stack::from(&coal_definition);
    coal_two.quantity = 69;

    let mut coal_three = Stack::from(&coal_definition);
    coal_three.quantity = 84;

    let mut inventory = Inventory::default();
    inventory.add_stack(&mut coal_one);
    info!("{:?}", inventory);
    inventory.add_stack(&mut coal_two);
    info!("{:?}", inventory);
    inventory.add_stack(&mut coal_three);
    info!("{:?}", inventory);
}
