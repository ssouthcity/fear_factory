use bevy::prelude::*;

use super::process::ProcessState;
use crate::gameplay::{inventory::prelude::*, recipe::assets::Recipe};

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
    mut item_definitions: ResMut<Assets<ItemDef>>,
    mut commands: Commands,
    mut recipe_changes: MessageWriter<RecipeChanged>,
) {
    let Some(recipe) = recipes.get(select_recipe.recipe) else {
        return;
    };

    commands
        .entity(select_recipe.entity)
        .despawn_related::<Inventory>();

    for (item_id, &requirement) in recipe.input.iter() {
        let handle = item_definitions.get_strong_handle(*item_id).unwrap();
        commands.spawn((
            item_stack_slot(select_recipe.entity, handle.clone(), 0),
            Input { requirement },
            DropOff::Item(handle),
        ));
    }

    for (item_id, &production) in recipe.output.iter() {
        let handle = item_definitions.get_strong_handle(*item_id).unwrap();
        commands.spawn((
            item_stack_slot(select_recipe.entity, handle, 0),
            Output { production },
            Pickup,
        ));
    }

    let Some(handle) = recipes.get_strong_handle(select_recipe.recipe) else {
        return;
    };

    commands
        .entity(select_recipe.entity)
        .insert(SelectedRecipe(handle));

    recipe_changes.write(RecipeChanged(select_recipe.entity));
}
