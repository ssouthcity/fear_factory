mod assets;
mod manifest;
mod process;
mod progress;
mod select;

use bevy::prelude::*;

pub use self::{
    assets::RecipeAssets,
    manifest::{Recipe, RecipeTags},
    process::ProcessState,
    select::{SelectRecipe, SelectedRecipe},
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        process::plugin,
        progress::plugin,
        select::plugin,
    ));
}
