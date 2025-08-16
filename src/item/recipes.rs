use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    assets::{
        LoadResource,
        manifest::{Id, Manifest, ManifestPlugin},
    },
    item::{Inventory, Item, ItemAssets, Stack},
    logistics::{InputInventory, OutputInventory},
    machine::work::Frequency,
};

pub fn plugin(app: &mut App) {
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();
    app.register_type::<RecipeAssets>();

    app.add_plugins(ManifestPlugin::<Recipe>::default())
        .load_resource::<RecipeAssets>();

    app.add_observer(on_select_recipe);
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct RecipeAssets {
    pub manifest: Handle<Manifest<Recipe>>,
}

impl FromWorld for RecipeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            manifest: assets.load("manifest/recipes.toml"),
        }
    }
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Recipe {
    pub name: String,
    #[serde(default)]
    pub input: HashMap<Id<Item>, u32>,
    #[serde(default)]
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
    for (id, quantity) in recipe.input.iter() {
        let def = item_manifest
            .get(id)
            .expect("Recipe refers to non-existent item");

        let slot = Stack::from(def).with_quantity(*quantity);

        input_inventory.add_slot(slot);
    }

    let mut output_inventory = Inventory::default();
    for (id, quantity) in recipe.output.iter() {
        let def = item_manifest
            .get(id)
            .expect("Recipe refers to non-existent item");

        let slot = Stack::from(def).with_quantity(*quantity);

        output_inventory.add_slot(slot);
    }

    commands.entity(trigger.target()).insert((
        SelectedRecipe(Some(event.0.clone())),
        InputInventory(input_inventory),
        OutputInventory(output_inventory),
        Frequency(recipe.duration),
    ));
}
