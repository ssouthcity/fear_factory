use bevy::prelude::*;

use crate::gameplay::{
    hud::tome::{TomeTab, UITomeLeftPageRoot, widgets},
    recipe::assets::RecipeDef,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::Recipes), spawn_recipe_list);

    app.add_systems(
        Update,
        refresh_recipe_plates.run_if(in_state(TomeTab::Recipes)),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RecipePlate(pub AssetId<RecipeDef>);

fn recipe_plate(recipe: AssetId<RecipeDef>) -> impl Bundle {
    (RecipePlate(recipe), Text::default())
}

fn spawn_recipe_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    recipe_defs: Res<Assets<RecipeDef>>,
) {
    let recipe_list = commands
        .spawn((
            widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::Recipes),
        ))
        .id();

    for (asset_id, _) in recipe_defs.iter() {
        commands.spawn((recipe_plate(asset_id), ChildOf(recipe_list)));
    }
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
