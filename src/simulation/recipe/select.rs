use bevy::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    simulation::{
        item::{Inventory, ItemDef, Stack},
        logistics::{InputInventory, OutputInventory},
        recipe::{ProcessState, assets::RecipeDef},
    },
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
    recipe_index: Res<IndexMap<RecipeDef>>,
    items: Res<Assets<ItemDef>>,
    item_index: Res<IndexMap<ItemDef>>,
    mut commands: Commands,
) {
    let event = trigger.event();

    let recipe_def = recipe_index
        .get(&event.0)
        .and_then(|asset_id| recipes.get(*asset_id))
        .expect("Attempted to select invalid recipe");

    let mut input_inventory = Inventory::default();
    for item_id in recipe_def.input.keys() {
        let item_def = item_index
            .get(item_id)
            .and_then(|asset_id| items.get(*asset_id))
            .expect("Recipe refers to non-existent item");

        let slot = Stack {
            item_id: item_def.id.to_owned(),
            quantity: 0,
            max_quantity: item_def.stack_size,
        };

        input_inventory.add_slot(slot);
    }

    let mut output_inventory = Inventory::default();
    for item_id in recipe_def.output.keys() {
        let item_def = item_index
            .get(item_id)
            .and_then(|asset_id| items.get(*asset_id))
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
