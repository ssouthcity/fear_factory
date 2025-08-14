use bevy::prelude::*;

mod assets;
mod collection;
mod item_id;
mod recipes;

pub use assets::ItemAssets;
pub use collection::ItemCollection;
pub use item_id::{ItemID, ItemLexicon};
pub use recipes::{Recipe, SelectRecipe, SelectedRecipe};
use serde::Deserialize;

use crate::assets::{ManifestParam, ManifestPlugin};

pub fn plugin(app: &mut App) {
    app.add_plugins((ManifestPlugin::<ItemDefinition>::new("manifest/items.toml"),));

    app.add_plugins((assets::plugin, collection::plugin, recipes::plugin));
}

#[derive(Debug, Deserialize, TypePath)]
pub struct ItemDefinition {
    name: String,
    stack_size: u32,
}
