use bevy::{prelude::*, ui_widgets::observe};

use crate::{
    gameplay::{
        item::inventory::Inventory,
        recipe::{
            assets::Recipe,
            select::{RecipeChanged, SelectRecipe},
        },
        tome::{
            UITomeLeftPageRoot, UITomeRightPageRoot,
            inspect::{InspectTabs, Inspected},
            list_page,
        },
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectTabs::RecipeSelect),
        (
            spawn_recipe_list,
            (spawn_recipe_details, refresh_recipe_details).chain(),
        ),
    );

    app.add_systems(
        Update,
        refresh_recipe_details.run_if(
            in_state(InspectTabs::RecipeSelect)
                .and(on_message::<RecipeChanged>.or(resource_changed::<Inspected>)),
        ),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RecipeButton(pub AssetId<Recipe>);

fn spawn_recipe_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    recipes: Res<Assets<Recipe>>,
) {
    let id = commands
        .spawn((
            list_page(),
            DespawnOnExit(InspectTabs::RecipeSelect),
            ChildOf(*left_page),
        ))
        .id();

    for (asset_id, _) in recipes.iter() {
        commands.spawn((
            widgets::recipe_plate(asset_id),
            ChildOf(id),
            RecipeButton(asset_id),
            observe(on_recipe_select),
        ));
    }
}

fn on_recipe_select(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    inspected: Res<Inspected>,
    recipe_badges: Query<&RecipeButton>,
) {
    let Ok(recipe) = recipe_badges.get(click.entity) else {
        return;
    };

    commands.trigger(SelectRecipe {
        entity: inspected.0,
        recipe: recipe.0,
    });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct RecipeDetails;

fn spawn_recipe_details(
    right_page: Single<Entity, With<UITomeRightPageRoot>>,
    mut commands: Commands,
) {
    commands.spawn((
        list_page(),
        RecipeDetails,
        DespawnOnExit(InspectTabs::RecipeSelect),
        ChildOf(*right_page),
    ));
}

fn refresh_recipe_details(
    inspected: Res<Inspected>,
    inventories: Query<&Inventory>,
    recipe_details: Single<Entity, With<RecipeDetails>>,
    mut commands: Commands,
) {
    commands.entity(*recipe_details).despawn_children();

    let Ok(inventory) = inventories.get(inspected.0) else {
        return;
    };

    for (id, _) in inventory.items.iter() {
        commands.spawn((
            widgets::item_plate(inspected.0, *id),
            ChildOf(*recipe_details),
        ));
    }
}
