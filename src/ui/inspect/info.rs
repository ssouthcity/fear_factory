use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

use crate::{
    assets::manifest::Manifest,
    item::{Recipe, RecipeAssets, SelectedRecipe},
    theme::widgets,
    ui::inspect::{InspectedEntity, InspectionMenuState},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectionMenuState::RecipeInspect),
        open_recipe_menu,
    );
}

pub fn open_recipe_menu(
    mut commands: Commands,
    inspected_entity: Res<InspectedEntity>,
    selected_recipes: Query<&SelectedRecipe>,
    // icons: Res<ItemAssets>,
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    recipe_assets: Res<RecipeAssets>,
) {
    let Ok(selected_recipe) = selected_recipes.get(inspected_entity.0) else {
        return;
    };

    let Some(ref recipe_id) = selected_recipe.0 else {
        return;
    };

    let manifest = recipe_manifests
        .get(&recipe_assets.manifest)
        .expect("Recipe manifest not loaded");

    let Some(recipe) = manifest.get(recipe_id) else {
        return;
    };

    let inputs: Vec<_> = recipe
        .input
        .iter()
        .map(|(item_id, quantity)| Text::new(format!("{:?} {}", item_id, quantity)))
        .collect();

    let outputs: Vec<_> = recipe
        .output
        .iter()
        .map(|(item_id, quantity)| Text::new(format!("{:?} {}", item_id, quantity)))
        .collect();

    commands.spawn((
        Name::new("Recipe Menu"),
        StateScoped(InspectionMenuState::RecipeInspect),
        widgets::container(),
        Children::spawn_one((
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(70.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(32.0)),
                ..default()
            },
            BackgroundColor(Color::WHITE.with_alpha(0.5)),
            Children::spawn((
                Spawn((
                    Node {
                        width: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Children::spawn((SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
                        parent
                            .spawn((Text::new("Deselect"), TextColor(Color::BLACK)))
                            .observe(on_deselect_recipe);

                        parent
                            .spawn((Text::new("Close"), TextColor(Color::BLACK)))
                            .observe(on_close_menu);
                    }),)),
                )),
                Spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Node {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            Children::spawn(inputs),
                        ),
                        (
                            TextLayout::new_with_justify(JustifyText::Center),
                            Text::new(format!("{} seconds", recipe.duration.as_secs_f32())),
                            TextColor(Color::BLACK),
                        ),
                        (
                            Node {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            Children::spawn(outputs),
                        ),
                    ],
                )),
            )),
        )),
    ));
}

fn on_deselect_recipe(
    _trigger: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
    mut selected_recipes: Query<&mut SelectedRecipe>,
    inspected_entity: Res<InspectedEntity>,
) {
    if let Ok(mut selected_recipe) = selected_recipes.get_mut(inspected_entity.0) {
        selected_recipe.0 = None;
    }

    next_state.set(InspectionMenuState::RecipeSelect);
}

fn on_close_menu(
    _trigger: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
) {
    next_state.set(InspectionMenuState::Closed);
}
