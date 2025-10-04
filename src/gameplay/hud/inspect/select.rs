use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

use crate::{
    gameplay::{
        hud::inspect::{InspectedEntity, InspectionMenuState},
        recipe::{assets::RecipeDef, select::SelectRecipe},
    },
    widgets,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectionMenuState::RecipeSelect),
        recipe_select_menu,
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SelectRecipeButton(String);

pub fn recipe_select_menu(mut commands: Commands, recipes: Res<Assets<RecipeDef>>) {
    let recipes = recipes
        .iter()
        // .filter(|(_, recipe)| {
        //     recipe
        //         .tags
        //         .contains(&RecipeTags::StructureId("constructor".to_string()))
        // })
        .map(|(_, recipe)| (recipe.id.to_owned(), recipe.name.to_owned()))
        .collect::<Vec<_>>();

    commands.spawn((
        Name::new("Recipe Selection Menu Container"),
        DespawnOnExit(InspectionMenuState::RecipeSelect),
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
                            .spawn((
                                Text::new("Close"),
                                TextColor(Color::BLACK),
                                Node {
                                    align_self: AlignSelf::End,
                                    ..default()
                                },
                            ))
                            .observe(on_close_menu);
                    }),)),
                )),
                Spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
                        for (recipe_id, recipe_name) in recipes {
                            parent
                                .spawn((
                                    Node {
                                        padding: UiRect::all(Val::Px(32.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::BLACK.with_alpha(0.5)),
                                    SelectRecipeButton(recipe_id.to_owned()),
                                    children![Text::new(recipe_name.to_string()),],
                                ))
                                .observe(on_recipe_hover)
                                .observe(on_recipe_click)
                                .observe(on_recipe_out);
                        }
                    })),
                )),
            )),
        )),
    ));
}

fn on_recipe_hover(pointer_over: On<Pointer<Over>>, mut commands: Commands) {
    commands
        .entity(pointer_over.entity)
        .insert(BackgroundColor(Color::hsl(120.0, 1.0, 0.5)));
}

fn on_recipe_out(pointer_out: On<Pointer<Out>>, mut commands: Commands) {
    commands
        .entity(pointer_out.entity)
        .insert(BackgroundColor(Color::BLACK.with_alpha(0.5)));
}

fn on_recipe_click(
    pointer_clicks: On<Pointer<Click>>,
    buttons: Query<&SelectRecipeButton>,
    inspected_entity: Res<InspectedEntity>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
    mut commands: Commands,
) {
    let Ok(button) = buttons.get(pointer_clicks.entity) else {
        return;
    };

    commands.trigger(SelectRecipe {
        entity: inspected_entity.0,
        recipe_id: button.0.clone(),
    });

    next_state.set(InspectionMenuState::RecipeInspect);
}

fn on_close_menu(
    _pointer_click: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
) {
    next_state.set(InspectionMenuState::Closed);
}
