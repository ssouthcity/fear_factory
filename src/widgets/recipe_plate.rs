use bevy::prelude::*;

use crate::gameplay::recipe::assets::Recipe;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, refresh_recipe_plates);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RecipePlate(pub AssetId<Recipe>);

pub fn recipe_plate(recipe: AssetId<Recipe>) -> impl Bundle {
    (RecipePlate(recipe), Text::default())
}

fn refresh_recipe_plates(plates: Query<(&RecipePlate, &mut Text)>, recipes: Res<Assets<Recipe>>) {
    for (plate, mut text) in plates {
        if let Some(recipe) = recipes.get(plate.0) {
            text.0 = recipe.name.clone();
        }
    }
}
