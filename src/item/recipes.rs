use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::item::{ItemCollection, ItemID};

pub fn plugin(app: &mut App) {
    app.register_type::<Recipe>();
    app.register_type::<RecipeCollection>();
    app.register_type::<SelectedRecipe>();
    app.register_type::<SelectRecipe>();

    app.init_resource::<RecipeCollection>();

    app.add_observer(on_select_recipe);

    app.add_systems(Startup, load_recipes);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct SelectedRecipe(pub Option<Recipe>);

#[derive(Event, Reflect)]
pub struct SelectRecipe(pub String);

fn on_select_recipe(
    trigger: Trigger<SelectRecipe>,
    recipe_collection: Res<RecipeCollection>,
    mut commands: Commands,
) {
    let event = trigger.event();

    let Some(recipe) = recipe_collection.get(&event.0) else {
        return;
    };

    commands
        .entity(trigger.target())
        .insert(SelectedRecipe(Some(recipe.clone())));
}

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct RecipeCollection(HashMap<String, Recipe>);

impl RecipeCollection {
    pub fn register_recipe(&mut self, name: String, recipe: Recipe) {
        self.0.insert(name, recipe);
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
        "Standard Iron".to_string(),
        Recipe {
            input: ItemCollection::new().with_item(ItemID("iron_ore"), 30),
            output: ItemCollection::new().with_item(ItemID("iron_ingot"), 30),
            duration: Duration::from_secs(10),
        },
    );

    recipe_collection.register_recipe(
        "Advanced Iron".to_string(),
        Recipe {
            input: ItemCollection::new()
                .with_item(ItemID("iron_ore"), 30)
                .with_item(ItemID("coal"), 30),
            output: ItemCollection::new().with_item(ItemID("iron_ingot"), 90),
            duration: Duration::from_secs(10),
        },
    );
}
