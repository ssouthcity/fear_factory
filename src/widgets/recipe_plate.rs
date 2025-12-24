use bevy::prelude::*;

use crate::gameplay::recipe::assets::RecipeDef;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, refresh_recipe_plates);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RecipePlate(pub AssetId<RecipeDef>);

pub fn recipe_plate(recipe: AssetId<RecipeDef>) -> impl Bundle {
    (RecipePlate(recipe), Text::default())
}

fn refresh_recipe_plates(
    plates: Query<(&RecipePlate, &mut Text)>,
    recipe_defs: Res<Assets<RecipeDef>>,
) {
    for (plate, mut text) in plates {
        if let Some(recipe) = recipe_defs.get(plate.0) {
            text.0 = recipe.name.clone();
        }
    }
}
