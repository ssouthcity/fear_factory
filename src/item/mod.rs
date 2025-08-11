use bevy::prelude::*;

mod assets;
mod collection;
mod item_id;
mod recipes;

pub use assets::ItemAssets;
pub use collection::ItemCollection;
pub use item_id::ItemID;
pub use recipes::{Recipe, RecipeCollection, RecipeID, SelectRecipe, SelectedRecipe};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        collection::plugin,
        item_id::plugin,
        recipes::plugin,
    ));
}
