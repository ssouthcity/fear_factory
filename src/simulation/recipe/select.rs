use bevy::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    simulation::{
        item::{Item, ItemDef, Quantity},
        recipe::{
            InputOf, Inputs, OutputOf, Outputs, ProcessState, RequiredQuantity, assets::RecipeDef,
        },
    },
};

pub fn plugin(app: &mut App) {
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();

    app.add_observer(on_select_recipe);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
#[require(ProcessState, Inputs, Outputs)]
pub struct SelectedRecipe(pub Option<String>);

#[derive(Event, Reflect)]
pub struct SelectRecipe(pub String);

fn on_select_recipe(
    trigger: Trigger<SelectRecipe>,
    recipes: Res<Assets<RecipeDef>>,
    recipe_index: Res<IndexMap<RecipeDef>>,
    mut items: ResMut<Assets<ItemDef>>,
    item_index: Res<IndexMap<ItemDef>>,
    mut commands: Commands,
) {
    let event = trigger.event();

    let recipe_def = recipe_index
        .get(&event.0)
        .and_then(|asset_id| recipes.get(*asset_id))
        .expect("Attempted to select invalid recipe");

    for (item_id, quantity) in recipe_def.input.iter() {
        let item_handle = item_index
            .get(item_id)
            .and_then(|asset_id| items.get_strong_handle(*asset_id))
            .expect("Recipe refers to non-existent item");

        commands.spawn((
            Name::new("Input"),
            ChildOf(trigger.target()),
            Item(item_handle),
            Quantity(0),
            RequiredQuantity(*quantity),
            InputOf(trigger.target()),
        ));
    }

    for (item_id, quantity) in recipe_def.output.iter() {
        let item_handle = item_index
            .get(item_id)
            .and_then(|asset_id| items.get_strong_handle(*asset_id))
            .expect("Recipe refers to non-existent item");

        commands.spawn((
            Name::new("Output"),
            ChildOf(trigger.target()),
            Item(item_handle),
            Quantity(0),
            RequiredQuantity(*quantity),
            OutputOf(trigger.target()),
        ));
    }

    commands
        .entity(trigger.target())
        .insert(SelectedRecipe(Some(event.0.clone())));
}
