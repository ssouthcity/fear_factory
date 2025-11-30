use bevy::prelude::*;

use super::process::ProcessState;
use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        item::{assets::ItemDef, stack::Stack},
        recipe::{Input, Output, assets::RecipeDef},
        storage::{Storage, StoredBy},
    },
};

pub fn plugin(app: &mut App) {
    app.add_message::<RecipeChanged>();

    app.add_observer(on_select_recipe);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
#[require(ProcessState)]
pub struct SelectedRecipe(pub String);

#[derive(EntityEvent, Reflect)]
pub struct SelectRecipe {
    pub entity: Entity,
    pub recipe_id: String,
}

#[derive(Message, Reflect)]
pub struct RecipeChanged(pub Entity);

fn on_select_recipe(
    select_recipe: On<SelectRecipe>,
    recipes: Res<Assets<RecipeDef>>,
    recipe_index: Res<IndexMap<RecipeDef>>,
    mut items: ResMut<Assets<ItemDef>>,
    item_index: Res<IndexMap<ItemDef>>,
    mut commands: Commands,
    mut recipe_changes: MessageWriter<RecipeChanged>,
) {
    let recipe_def = recipe_index
        .get(&select_recipe.recipe_id)
        .and_then(|asset_id| recipes.get(*asset_id))
        .expect("Attempted to select invalid recipe");

    commands
        .entity(select_recipe.entity)
        .despawn_related::<Storage>();

    for (item_id, quantity) in recipe_def.input.iter() {
        let item_handle = item_index
            .get(item_id)
            .and_then(|asset_id| items.get_strong_handle(*asset_id))
            .expect("Recipe refers to non-existent item");

        commands.spawn((
            Name::new("Input"),
            ChildOf(select_recipe.entity),
            StoredBy(select_recipe.entity),
            Stack::empty(item_handle),
            Input {
                quantity: *quantity,
            },
        ));
    }

    for (item_id, quantity) in recipe_def.output.iter() {
        let item_handle = item_index
            .get(item_id)
            .and_then(|asset_id| items.get_strong_handle(*asset_id))
            .expect("Recipe refers to non-existent item");

        commands.spawn((
            Name::new("Output"),
            ChildOf(select_recipe.entity),
            StoredBy(select_recipe.entity),
            Stack::empty(item_handle),
            Output {
                quantity: *quantity,
            },
        ));
    }

    commands
        .entity(select_recipe.entity)
        .insert(SelectedRecipe(select_recipe.recipe_id.to_owned()));

    recipe_changes.write(RecipeChanged(select_recipe.entity));
}
