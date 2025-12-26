use bevy::prelude::*;

use super::process::ProcessState;
use crate::gameplay::{item::inventory::Inventory, recipe::assets::Recipe};

pub fn plugin(app: &mut App) {
    app.add_message::<RecipeChanged>();

    app.add_observer(on_select_recipe);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
#[require(ProcessState)]
pub struct SelectedRecipe(pub Handle<Recipe>);

#[derive(EntityEvent, Reflect)]
pub struct SelectRecipe {
    pub entity: Entity,
    pub recipe: AssetId<Recipe>,
}

#[derive(Message, Reflect)]
pub struct RecipeChanged(pub Entity);

fn on_select_recipe(
    select_recipe: On<SelectRecipe>,
    mut recipes: ResMut<Assets<Recipe>>,
    mut commands: Commands,
    mut recipe_changes: MessageWriter<RecipeChanged>,
) {
    let Some(recipe) = recipes.get(select_recipe.recipe) else {
        return;
    };

    let mut inventory = Inventory::default();

    for (item_id, _) in recipe.input.iter().chain(recipe.output.iter()) {
        inventory.items.insert(*item_id, 0);
    }

    let Some(handle) = recipes.get_strong_handle(select_recipe.recipe) else {
        return;
    };

    commands
        .entity(select_recipe.entity)
        .insert((SelectedRecipe(handle), inventory));

    recipe_changes.write(RecipeChanged(select_recipe.entity));
}
