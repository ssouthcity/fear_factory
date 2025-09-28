use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        hud::{
            inspect::{InspectedEntity, InspectionMenuState},
            item_slot::{AddedToSlot, InSlot, RemovedFromSlot},
        },
        item::{Item, assets::ItemDef},
        recipe::{assets::RecipeDef, select::SelectedRecipe},
    },
    widgets::{self, item::item_icon},
};

pub fn plugin(app: &mut App) {
    app.register_type::<HeldRelic>();

    app.add_systems(
        OnEnter(InspectionMenuState::RecipeInspect),
        open_recipe_menu,
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HeldRelic(Handle<ItemDef>);

#[allow(clippy::too_many_arguments)]
pub fn open_recipe_menu(
    mut commands: Commands,
    inspected_entity: Res<InspectedEntity>,
    selected_recipes: Query<&SelectedRecipe>,
    recipes: Res<Assets<RecipeDef>>,
    recipe_index: Res<IndexMap<RecipeDef>>,
    held_relics: Query<&HeldRelic>,
    asset_server: Res<AssetServer>,
) {
    let Ok(selected_recipe) = selected_recipes.get(inspected_entity.0) else {
        return;
    };

    let Some(ref recipe_id) = selected_recipe.0 else {
        return;
    };

    let Some(recipe) = recipe_index
        .get(recipe_id)
        .and_then(|asset_id| recipes.get(*asset_id))
    else {
        return;
    };

    let container_id = commands
        .spawn((
            Name::new("Recipe Menu"),
            StateScoped(InspectionMenuState::RecipeInspect),
            Pickable::IGNORE,
            widgets::container(),
        ))
        .id();

    let menu_id = commands
        .spawn((
            ChildOf(container_id),
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(70.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(32.0)),
                ..default()
            },
            BackgroundColor(Color::WHITE.with_alpha(0.5)),
        ))
        .id();

    commands.spawn((
        ChildOf(menu_id),
        Node {
            width: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            parent
                .spawn((Text::new("Deselect"), TextColor(Color::BLACK)))
                .observe(on_deselect_recipe);

            parent
                .spawn((Text::new("Close"), TextColor(Color::BLACK)))
                .observe(on_close_menu);
        })),
    ));

    let recipe_view_id = commands
        .spawn((
            ChildOf(menu_id),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    let input_list_id = commands
        .spawn((
            ChildOf(recipe_view_id),
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .id();

    for (input_id, quantity) in recipe.input.iter() {
        commands.spawn((
            ChildOf(input_list_id),
            widgets::slot(),
            children![
                item_icon(asset_server.load(format!("manifests/items/{input_id}.item.toml"))),
                (
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::ZERO,
                        bottom: Val::ZERO,
                        ..default()
                    },
                    Pickable::IGNORE,
                    BackgroundColor(Color::WHITE),
                    TextColor(Color::BLACK),
                    Text::new(quantity.to_string()),
                )
            ],
        ));
    }

    let middle_column_id = commands
        .spawn((
            ChildOf(recipe_view_id),
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(middle_column_id),
        TextLayout::new_with_justify(JustifyText::Center),
        Text::new(format!("{} seconds", recipe.duration.as_secs_f32())),
        TextColor(Color::BLACK),
    ));

    let relic_slot_id = commands
        .spawn((ChildOf(middle_column_id), widgets::slot()))
        .id();

    if let Ok(relic) = held_relics.get(inspected_entity.0) {
        commands.spawn((
            InSlot(relic_slot_id),
            ChildOf(relic_slot_id),
            item_icon(relic.0.clone()),
        ));
    }

    commands
        .entity(relic_slot_id)
        .observe(
            |trigger: Trigger<AddedToSlot>,
             inspected_entity: Res<InspectedEntity>,
             mut commands: Commands,
             items: Query<&Item>| {
                if let Ok(item) = items.get(trigger.0) {
                    commands
                        .entity(inspected_entity.0)
                        .insert(HeldRelic(item.0.clone()));
                }
            },
        )
        .observe(
            |_trigger: Trigger<RemovedFromSlot>,
             inspected_entity: Res<InspectedEntity>,
             mut commands: Commands| {
                commands.entity(inspected_entity.0).remove::<HeldRelic>();
            },
        );

    let output_list_id = commands
        .spawn((
            ChildOf(recipe_view_id),
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .id();

    for (output_id, quantity) in recipe.output.iter() {
        commands.spawn((
            ChildOf(output_list_id),
            widgets::slot(),
            children![
                item_icon(asset_server.load(format!("manifests/items/{output_id}.item.toml"))),
                (
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::ZERO,
                        bottom: Val::ZERO,
                        ..default()
                    },
                    Pickable::IGNORE,
                    BackgroundColor(Color::WHITE),
                    TextColor(Color::BLACK),
                    Text::new(quantity.to_string()),
                )
            ],
        ));
    }
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
