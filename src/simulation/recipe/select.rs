use bevy::prelude::*;

use crate::simulation::{
    item::{Inventory, ItemDef, Stack},
    logistics::{InputInventory, OutputInventory},
    recipe::{ProcessState, assets::RecipeDef},
};

pub fn plugin(app: &mut App) {
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();

    app.add_observer(on_select_recipe);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
#[require(ProcessState, InputInventory, OutputInventory)]
pub struct SelectedRecipe(pub Option<String>);

#[derive(Event, Reflect)]
pub struct SelectRecipe(pub String);

fn on_select_recipe(
    trigger: Trigger<SelectRecipe>,
    recipes: Res<Assets<RecipeDef>>,
    items: Res<Assets<ItemDef>>,
    mut commands: Commands,
) {
    let event = trigger.event();

    let Some(recipe_def) = recipes
        .iter()
        .map(|(_, recipe)| recipe)
        .find(|recipe| recipe.id == event.0)
    else {
        warn!("Attempted to select invalid recipe");
        return;
    };

    let mut input_inventory = Inventory::default();
    for id in recipe_def.input.keys() {
        let item_def = items
            .iter()
            .map(|(_, item)| item)
            .find(|item| item.id == *id)
            .expect("Recipe refers to non-existent item");

        let slot = Stack {
            item_id: item_def.id.to_owned(),
            quantity: 0,
            max_quantity: item_def.stack_size,
        };

        input_inventory.add_slot(slot);
    }

    let mut output_inventory = Inventory::default();
    for id in recipe_def.output.keys() {
        let item_def = items
            .iter()
            .map(|(_, item)| item)
            .find(|item| item.id == *id)
            .expect("Recipe refers to non-existent item");

        let slot = Stack {
            item_id: item_def.id.to_owned(),
            quantity: 0,
            max_quantity: item_def.stack_size,
        };

        output_inventory.add_slot(slot);
    }

    commands.entity(trigger.target()).insert((
        SelectedRecipe(Some(event.0.clone())),
        InputInventory(input_inventory),
        OutputInventory(output_inventory),
    ));
}
