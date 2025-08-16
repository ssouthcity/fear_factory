use bevy::prelude::*;

use crate::{
    FactorySystems,
    assets::manifest::Manifest,
    item::{Inventory, Item, ItemAssets, Recipe, RecipeAssets, SelectedRecipe, Stack},
    machine::{
        Machine,
        work::{BeginWork, WorkCompleted, Working},
    },
};

pub fn plugin(app: &mut App) {
    app.register_type::<InputInventory>();
    app.register_type::<OutputInventory>();

    app.add_systems(
        Update,
        (begin_work, move_output_to_output_inventory)
            .chain()
            .in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct InputInventory(pub Inventory);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct OutputInventory(pub Inventory);

fn begin_work(
    mut events: EventWriter<BeginWork>,
    machines: Query<
        (Entity, &SelectedRecipe, &mut InputInventory),
        (With<Machine>, Without<Working>),
    >,
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    recipe_assets: Res<RecipeAssets>,
) {
    for (entity, selected_recipe, mut inventory) in machines {
        let Some(ref recipe_id) = selected_recipe.0 else {
            continue;
        };

        let recipe = recipe_manifests
            .get(&recipe_assets.manifest)
            .expect("Recipe manifest not loaded")
            .get(recipe_id)
            .expect("Selected recipe refers to non-existent id");

        if inventory.consume_input(recipe).is_ok() {
            events.write(BeginWork(entity));
        };
    }
}

fn move_output_to_output_inventory(
    mut events: EventReader<WorkCompleted>,
    mut outputs: Query<(&SelectedRecipe, &mut OutputInventory)>,
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    recipe_assets: Res<RecipeAssets>,
    item_manifests: Res<Assets<Manifest<Item>>>,
    item_assets: Res<ItemAssets>,
) {
    for event in events.read() {
        let Ok((selected_recipe, mut output_inventory)) = outputs.get_mut(event.0) else {
            continue;
        };

        let Some(ref recipe_id) = selected_recipe.0 else {
            continue;
        };

        let recipe = recipe_manifests
            .get(&recipe_assets.manifest)
            .expect("Recipe manifest not loaded")
            .get(recipe_id)
            .expect("Selected recipe refers to non-existent id");

        let items = item_manifests
            .get(&item_assets.manifest)
            .expect("Item manifest not loaded");

        let mut stacks: Vec<Stack> = recipe
            .output
            .iter()
            .map(|(item_id, quantity)| {
                let item = items
                    .get(item_id)
                    .expect("Recipe refers to non-existent item");

                Stack::from(item).with_quantity(*quantity)
            })
            .collect();

        for stack in stacks.iter_mut() {
            let _ = output_inventory.add_stack(stack);
        }
    }
}
