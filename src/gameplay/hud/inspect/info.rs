use bevy::{prelude::*, ui_widgets::observe};

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        hud::inspect::{InspectedEntity, InspectionMenuState},
        recipe::{Inputs, Outputs, assets::RecipeDef, select::SelectedRecipe},
    },
    widgets::{
        self,
        item::StackIcon,
        slot::{AddedToSlot, RemovedFromSlot},
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectionMenuState::RecipeInspect),
        open_recipe_menu,
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HeldRelic(Entity);

#[allow(clippy::too_many_arguments)]
pub fn open_recipe_menu(
    mut commands: Commands,
    inspected_entity: Res<InspectedEntity>,
    structure_query: Query<(&SelectedRecipe, &Inputs, &Outputs)>,
    recipes: Res<Assets<RecipeDef>>,
    recipe_index: Res<IndexMap<RecipeDef>>,
    held_relics: Query<&HeldRelic>,
) {
    let Ok((selected_recipe, inputs, outputs)) = structure_query.get(inspected_entity.0) else {
        return;
    };

    let Some(recipe) = recipe_index
        .get(selected_recipe.0.as_str())
        .and_then(|asset_id| recipes.get(*asset_id))
    else {
        return;
    };

    let container_id = commands
        .spawn((
            Name::new("Recipe Menu"),
            DespawnOnExit(InspectionMenuState::RecipeInspect),
            Pickable::IGNORE,
            widgets::container(),
        ))
        .id();

    let menu_id = commands
        .spawn((
            ChildOf(container_id),
            Node {
                width: percent(70.0),
                height: percent(70.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: px(32.0).all(),
                ..default()
            },
            BackgroundColor(Color::WHITE.with_alpha(0.5)),
        ))
        .id();

    commands.spawn((
        ChildOf(menu_id),
        Node {
            width: percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new("Deselect"),
                TextColor(Color::BLACK),
                observe(on_deselect_recipe),
            ),
            (
                Text::new("Close"),
                TextColor(Color::BLACK),
                observe(on_close_menu),
            )
        ],
    ));

    let recipe_view_id = commands
        .spawn((
            ChildOf(menu_id),
            Node {
                width: percent(100.0),
                height: percent(100.0),
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

    for input in inputs.iter() {
        let slot = commands
            .spawn((ChildOf(input_list_id), widgets::slot::slot_container()))
            .id();
        commands.spawn(widgets::slot::slotted_stack(slot, input));
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
        TextLayout::new_with_justify(Justify::Center),
        Text::new(format!("{} seconds", recipe.duration.as_secs_f32())),
        TextColor(Color::BLACK),
    ));

    let relic_slot_id = commands
        .spawn((
            ChildOf(middle_column_id),
            widgets::slot::slot_container(),
            observe(
                |added_to_slot: On<AddedToSlot>,
                 inspected_entity: Res<InspectedEntity>,
                 mut commands: Commands,
                 slot_occupant_query: Query<&Children>,
                 stack_icon_query: Query<&StackIcon>| {
                    let Ok(children) = slot_occupant_query.get(added_to_slot.item) else {
                        return;
                    };

                    let Ok(stack_icon) = stack_icon_query.get(*children.first().unwrap()) else {
                        return;
                    };

                    commands
                        .entity(inspected_entity.0)
                        .insert(HeldRelic(stack_icon.0));
                },
            ),
            observe(
                |_removed_from_slot: On<RemovedFromSlot>,
                 inspected_entity: Res<InspectedEntity>,
                 mut commands: Commands| {
                    commands.entity(inspected_entity.0).remove::<HeldRelic>();
                },
            ),
        ))
        .id();

    if let Ok(relic) = held_relics.get(inspected_entity.0) {
        commands.spawn(widgets::slot::slotted_stack(relic_slot_id, relic.0));
    }

    let output_list_id = commands
        .spawn((
            ChildOf(recipe_view_id),
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .id();

    for output in outputs.iter() {
        let slot = commands
            .spawn((ChildOf(output_list_id), widgets::slot::slot_container()))
            .id();
        commands.spawn(widgets::slot::slotted_stack(slot, output));
    }
}

fn on_deselect_recipe(
    _pointer_click: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
    inspected_entity: Res<InspectedEntity>,
    mut commands: Commands,
) {
    commands
        .entity(inspected_entity.0)
        .remove::<SelectedRecipe>();

    next_state.set(InspectionMenuState::RecipeSelect);
}

fn on_close_menu(
    _pointer_click: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
) {
    next_state.set(InspectionMenuState::Closed);
}
