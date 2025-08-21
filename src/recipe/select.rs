use bevy::prelude::*;

use super::{Recipe, RecipeAssets};

use crate::{
    assets::manifest::{Id, Manifest},
    item::{Inventory, Item, ItemAssets, Stack},
    logistics::{InputInventory, OutputInventory},
    recipe::ProcessState,
};

pub fn plugin(app: &mut App) {
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();

    app.add_observer(on_select_recipe);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
#[require(ProcessState, InputInventory, OutputInventory)]
pub struct SelectedRecipe(pub Option<Id<Recipe>>);

#[derive(Event, Reflect)]
pub struct SelectRecipe(pub Id<Recipe>);

fn on_select_recipe(
    trigger: Trigger<SelectRecipe>,
    recipe_assets: Res<RecipeAssets>,
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    item_assets: Res<ItemAssets>,
    item_manifests: Res<Assets<Manifest<Item>>>,
    mut commands: Commands,
) {
    let event = trigger.event();

    let recipe_manifest = recipe_manifests
        .get(&recipe_assets.manifest)
        .expect("Recipe manifests not loaded");

    let item_manifest = item_manifests
        .get(&item_assets.manifest)
        .expect("Item manifests not loaded");

    let Some(recipe) = recipe_manifest.get(&event.0) else {
        warn!("Attempted to select invalid recipe");
        return;
    };

    let mut input_inventory = Inventory::default();
    for id in recipe.input.keys() {
        let def = item_manifest
            .get(id)
            .expect("Recipe refers to non-existent item");

        let slot = Stack::from(def);

        input_inventory.add_slot(slot);
    }

    let mut output_inventory = Inventory::default();
    for id in recipe.output.keys() {
        let def = item_manifest
            .get(id)
            .expect("Recipe refers to non-existent item");

        let slot = Stack::from(def);

        output_inventory.add_slot(slot);
    }

    commands.entity(trigger.target()).insert((
        SelectedRecipe(Some(event.0.clone())),
        InputInventory(input_inventory),
        OutputInventory(output_inventory),
    ));
}
