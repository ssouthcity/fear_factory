mod assets;
mod manifest;
mod select;

use bevy::prelude::*;

pub use self::{
    assets::RecipeAssets,
    manifest::{Recipe, RecipeTags},
    select::{SelectRecipe, SelectedRecipe},
};

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, select::plugin));
}
