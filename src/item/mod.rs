use bevy::prelude::*;
use serde::Deserialize;

mod assets;
mod inventory;
mod recipes;
mod stack;

pub use assets::ItemAssets;
pub use inventory::Inventory;
pub use recipes::{Recipe, SelectRecipe, SelectedRecipe};
pub use stack::Stack;

use crate::assets::manifest::ManifestPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins((ManifestPlugin::<Item>::new("manifest/items.toml"),));

    app.add_plugins((assets::plugin, recipes::plugin));
}

#[derive(Debug, Deserialize, TypePath)]
#[allow(dead_code)]
pub struct Item {
    name: String,
    stack_size: u32,
}
