use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

use crate::{
    assets::manifest::{Id, ManifestParam},
    item::{Recipe, SelectRecipe},
    theme::widgets,
    ui::inspect::{InspectedEntity, InspectionMenuState},
};

pub fn plugin(app: &mut App) {
    app.register_type::<SelectRecipeButton>();

    app.add_systems(
        OnEnter(InspectionMenuState::RecipeSelect),
        recipe_select_menu,
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SelectRecipeButton(Id<Recipe>);

pub fn recipe_select_menu(mut commands: Commands, recipe_manifest: ManifestParam<Recipe>) {
    let Some(manifest) = recipe_manifest.read() else {
        return;
    };

    let recipes = manifest
        .iter()
        .map(|(id, recipe)| (id.to_owned(), recipe.name.to_owned()))
        .collect::<Vec<_>>();

    commands.spawn((
        Name::new("Recipe Selection Menu Container"),
        StateScoped(InspectionMenuState::RecipeSelect),
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
    trigger: Trigger<Pointer<Click>>,
    buttons: Query<&SelectRecipeButton>,
    inspected_entity: Res<InspectedEntity>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
    mut commands: Commands,
) {
    let Ok(button) = buttons.get(trigger.target()) else {
        return;
    };

    commands.trigger_targets(SelectRecipe(button.0.clone()), inspected_entity.0);

    next_state.set(InspectionMenuState::RecipeInspect);
}

fn on_close_menu(
    _trigger: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
) {
    next_state.set(InspectionMenuState::Closed);
}
