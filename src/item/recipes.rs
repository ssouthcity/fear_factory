use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    item::{ItemCollection, ItemID},
    logistics::{InputFilter, ResourceInput, ResourceOutput},
    machine::work::Frequency,
};

pub fn plugin(app: &mut App) {
    app.register_type::<RecipeID>();
    app.register_type::<Recipe>();
    app.register_type::<RecipeCollection>();
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();

    app.init_resource::<RecipeCollection>();

    app.add_observer(on_select_recipe);

    app.add_systems(Startup, load_recipes);
}

#[derive(Component, Hash, PartialEq, Eq, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct RecipeID(pub &'static str);

impl std::fmt::Display for RecipeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RecipeID({})", self.0)
    }
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct SelectedRecipe(pub Option<RecipeID>);

#[derive(Event, Reflect)]
pub struct SelectRecipe(pub RecipeID);

fn on_select_recipe(
    trigger: Trigger<SelectRecipe>,
    recipe_collection: Res<RecipeCollection>,
    mut commands: Commands,
) {
    let event = trigger.event();

    let Some(recipe) = recipe_collection.get(&event.0) else {
        warn!("Attempted to select invalid recipe");
        return;
    };

    let mut input_filter = InputFilter::default();
    for item_id in recipe.input.keys() {
        input_filter.insert(*item_id);
    }

    commands.entity(trigger.target()).insert((
        SelectedRecipe(Some(event.0)),
        ResourceInput(recipe.input.clone()),
        ResourceOutput(recipe.output.clone()),
        Frequency(recipe.duration),
        input_filter,
    ));
}

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct RecipeCollection(HashMap<RecipeID, Recipe>);

impl RecipeCollection {
    pub fn register_recipe(&mut self, recipe_id: RecipeID, recipe: Recipe) {
        self.0.insert(recipe_id, recipe);
    }
}

#[derive(Reflect, Clone)]
pub struct Recipe {
    pub input: ItemCollection,
    pub duration: Duration,
    pub output: ItemCollection,
}

pub fn load_recipes(mut recipe_collection: ResMut<RecipeCollection>) {
    recipe_collection.register_recipe(
        RecipeID("standard_iron"),
        Recipe {
            input: ItemCollection::new().with_item(ItemID("iron_ore"), 30),
            output: ItemCollection::new().with_item(ItemID("iron_ingot"), 30),
            duration: Duration::from_secs(10),
        },
    );

    recipe_collection.register_recipe(
        RecipeID("iron_plates"),
        Recipe {
            input: ItemCollection::new().with_item(ItemID("iron_ingot"), 30),
            output: ItemCollection::new().with_item(ItemID("iron_plate"), 10),
            duration: Duration::from_secs(10),
        },
    );
}
