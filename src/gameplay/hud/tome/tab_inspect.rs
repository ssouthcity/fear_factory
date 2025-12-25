use bevy::{prelude::*, ui_widgets::observe};

use crate::{
    gameplay::{
        hud::tome::{TomeOpen, TomeTab, UITomeLeftPageRoot, UITomeRightPageRoot},
        item::inventory::Inventory,
        recipe::{
            assets::RecipeDef,
            select::{RecipeChanged, SelectRecipe},
        },
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(TomeTab::Inspect),
        (spawn_inspect_recipes, spawn_right_page),
    );

    app.add_systems(
        Update,
        spawn_right_page.run_if(
            in_state(TomeTab::Inspect)
                .and(on_message::<RecipeChanged>.or(resource_changed::<Inspected>)),
        ),
    );

    app.add_observer(on_inspect);
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct Inspected(pub Entity);

#[derive(EntityEvent, Reflect)]
pub struct Inspect {
    pub entity: Entity,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Recipe(pub AssetId<RecipeDef>);

fn on_inspect(
    inspect: On<Inspect>,
    mut commands: Commands,
    mut next_tome_open: ResMut<NextState<TomeOpen>>,
    mut next_tome_tab: ResMut<NextState<TomeTab>>,
) {
    commands.insert_resource(Inspected(inspect.entity));
    next_tome_open.set(TomeOpen(true));
    next_tome_tab.set(TomeTab::Inspect);
}

fn spawn_inspect_recipes(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    recipes: Res<Assets<RecipeDef>>,
) {
    let id = commands
        .spawn((
            super::widgets::list_page(),
            DespawnOnExit(TomeTab::Inspect),
            ChildOf(*left_page),
        ))
        .id();

    for (asset_id, _) in recipes.iter() {
        commands.spawn((
            widgets::recipe_plate(asset_id),
            ChildOf(id),
            Recipe(asset_id),
            observe(on_recipe_select),
        ));
    }
}

fn on_recipe_select(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    inspected: Res<Inspected>,
    recipe_badges: Query<&Recipe>,
) {
    let Ok(recipe) = recipe_badges.get(click.entity) else {
        return;
    };

    commands.trigger(SelectRecipe {
        entity: inspected.0,
        recipe: recipe.0,
    });
}

fn spawn_right_page(
    inspected: Res<Inspected>,
    inventories: Query<&Inventory>,
    right_page: Single<Entity, With<UITomeRightPageRoot>>,
    mut commands: Commands,
) {
    let Ok(inventory) = inventories.get(inspected.0) else {
        return;
    };

    commands.entity(*right_page).despawn_children();

    let list = commands
        .spawn((
            super::widgets::list_page(),
            DespawnOnExit(TomeTab::Inspect),
            ChildOf(*right_page),
        ))
        .id();

    for (id, _) in inventory.items.iter() {
        commands.spawn((widgets::item_plate(inspected.0, *id), ChildOf(list)));
    }
}
