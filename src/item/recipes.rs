use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    assets::manifest::{Id, ManifestParam, ManifestPlugin},
    item::{Item, Stack},
    logistics::{ResourceInput, ResourceOutput},
    machine::work::Frequency,
};

pub fn plugin(app: &mut App) {
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();

    app.add_plugins(ManifestPlugin::<Recipe>::new("manifest/recipes.toml"));

    app.add_observer(on_select_recipe);
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Recipe {
    pub name: String,
    pub input: HashMap<Id<Item>, u32>,
    pub output: HashMap<Id<Item>, u32>,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct SelectedRecipe(pub Option<Id<Recipe>>);

#[derive(Event, Reflect)]
pub struct SelectRecipe(pub Id<Recipe>);

fn on_select_recipe(
    trigger: Trigger<SelectRecipe>,
    recipe_manifest: ManifestParam<Recipe>,
    item_manifest: ManifestParam<Item>,
    mut commands: Commands,
) {
    let event = trigger.event();

    let Some(recipes) = recipe_manifest.read() else {
        warn!("Recipes have not been loaded");
        return;
    };

    let Some(items) = item_manifest.read() else {
        warn!("Items have not been loaded");
        return;
    };

    let Some(recipe) = recipes.get(&event.0) else {
        warn!("Attempted to select invalid recipe");
        return;
    };

    let input_items = recipe
        .input
        .iter()
        .map(|(id, quantity)| {
            let def = items.get(id).expect("Recipe refers to non-existent item");
            Stack::from(&def).with_quantity(*quantity)
        })
        .collect();

    let output_items = recipe
        .output
        .iter()
        .map(|(id, quantity)| {
            let def = items.get(id).expect("Recipe refers to non-existent item");
            Stack::from(&def).with_quantity(*quantity)
        })
        .collect();

    commands.entity(trigger.target()).insert((
        SelectedRecipe(Some(event.0.clone())),
        ResourceInput(input_items),
        ResourceOutput(output_items),
        Frequency(recipe.duration),
    ));
}
