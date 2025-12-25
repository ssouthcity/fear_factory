use bevy::prelude::*;

use super::process::ProcessState;
use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        item::{assets::ItemDef, inventory::Inventory},
        recipe::assets::RecipeDef,
    },
};

pub fn plugin(app: &mut App) {
    app.add_message::<RecipeChanged>();

    app.add_observer(on_select_recipe);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
#[require(ProcessState)]
pub struct SelectedRecipe(pub Handle<RecipeDef>);

#[derive(EntityEvent, Reflect)]
pub struct SelectRecipe {
    pub entity: Entity,
    pub recipe: AssetId<RecipeDef>,
}

#[derive(Message, Reflect)]
pub struct RecipeChanged(pub Entity);

fn on_select_recipe(
    select_recipe: On<SelectRecipe>,
    mut recipes: ResMut<Assets<RecipeDef>>,
    item_index: Res<IndexMap<ItemDef>>,
    mut commands: Commands,
    mut recipe_changes: MessageWriter<RecipeChanged>,
) {
    let Some(recipe_def) = recipes.get(select_recipe.recipe) else {
        return;
    };

    let mut inventory = Inventory::default();

    for item_id in recipe_def
        .input
        .iter()
        .chain(recipe_def.output.iter())
        .flat_map(|(id, _)| item_index.get(id))
    {
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
