use std::time::Duration;

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    assets::manifest::{Id, ManifestParam, ManifestPlugin},
    item::Item,
    logistics::InputFilter,
    machine::work::Frequency,
};

pub fn plugin(app: &mut App) {
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();

    app.add_plugins(ManifestPlugin::<Recipe>::new("manifest/recipes.toml"));

    app.add_observer(on_select_recipe);
}

#[derive(Debug, Deserialize, TypePath)]
#[allow(dead_code)]
pub struct Recipe {
    pub name: String,
    pub input: Vec<(Id<Item>, u32)>,
    pub output: Vec<(Id<Item>, u32)>,
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
    mut commands: Commands,
) {
    let event = trigger.event();

    let Some(manifest) = recipe_manifest.get() else {
        warn!("Recipes have not been loaded");
        return;
    };

    let Some(recipe) = manifest.get(&event.0) else {
        warn!("Attempted to select invalid recipe");
        return;
    };

    let mut input_filter = InputFilter::default();
    for (item_id, _) in recipe.input.iter() {
        input_filter.insert(item_id.clone().into());
    }

    commands.entity(trigger.target()).insert((
        SelectedRecipe(Some(event.0.clone())),
        // ResourceInput(recipe.input.clone()),
        // ResourceOutput(recipe.output.clone()),
        Frequency(recipe.duration),
        input_filter,
    ));
}
