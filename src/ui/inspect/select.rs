use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

use crate::{
    item::{RecipeCollection, SelectedRecipe},
    theme::widgets,
    ui::inspect::{InspectedEntity, InspectionMenuState},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectionMenuState::RecipeSelect),
        recipe_select_menu,
    );
}

pub fn recipe_select_menu(mut commands: Commands, recipes: Res<RecipeCollection>) {
    let recipes = recipes.keys().map(|n| n.to_owned()).collect::<Vec<_>>();

    commands.spawn((
        Name::new("Recipe Selection Menu Container"),
        StateScoped(InspectionMenuState::RecipeSelect),
        widgets::container(),
        Children::spawn_one((
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(70.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                padding: UiRect::all(Val::Px(32.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
                for recipe in recipes {
                    parent
                        .spawn((
                            Node {
                                padding: UiRect::all(Val::Px(32.0)),
                                ..default()
                            },
                            BackgroundColor(Color::BLACK.with_alpha(0.5)),
                            children![Text::new(recipe.to_owned()),],
                        ))
                        .observe(on_recipe_hover)
                        .observe(on_recipe_click)
                        .observe(on_recipe_out);
                }
            })),
        )),
    ));
}

fn on_recipe_hover(trigger: Trigger<Pointer<Over>>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(BackgroundColor(Color::hsl(120.0, 1.0, 0.5)));
}

fn on_recipe_out(trigger: Trigger<Pointer<Out>>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(BackgroundColor(Color::BLACK.with_alpha(0.5)));
}

fn on_recipe_click(
    _trigger: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
    mut selected_recipes: Query<&mut SelectedRecipe>,
    inspected_entity: Res<InspectedEntity>,
    recipes: Res<RecipeCollection>,
) {
    let Ok(mut selected_recipe) = selected_recipes.get_mut(inspected_entity.0) else {
        return;
    };

    let Some(recipe) = recipes.get("Standard Iron") else {
        return;
    };

    selected_recipe.0 = Some(recipe.to_owned());

    next_state.set(InspectionMenuState::RecipeInspect);
}
