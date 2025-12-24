use bevy::{
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioButton, ValueChange, observe},
};

use crate::{
    gameplay::{
        hud::tome::widgets::{TAB_COLOR_CHECKED, TAB_COLOR_DEFAULT},
        recipe::assets::RecipeDef,
    },
    input::input_map::{Action, action_just_pressed},
    screens::Screen,
};

pub mod tab_inspect;
pub mod tab_items;
pub mod tab_people;
pub mod tab_recipes;
pub mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        tab_inspect::plugin,
        tab_items::plugin,
        tab_people::plugin,
        tab_recipes::plugin,
    ));

    app.add_sub_state::<TomeOpen>();
    app.add_sub_state::<TomeTab>();
    app.init_resource::<TomeFocused>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_ui_root, spawn_tome_root).chain(),
    );

    app.add_systems(
        Update,
        (
            toggle_tome_open.run_if(action_just_pressed(Action::OpenTome)),
            set_tome_visibility.run_if(run_once.or(state_changed::<TomeOpen>)),
        )
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        Update,
        (refresh_tab_style, update_tab_color)
            .chain()
            .run_if(state_changed::<TomeTab>),
    );
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(Screen = Screen::Gameplay)]
pub struct TomeOpen(pub bool);

#[derive(SubStates, Component, Reflect, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
#[reflect(Component)]
#[source(Screen = Screen::Gameplay)]
pub enum TomeTab {
    #[default]
    Items,
    People,
    Recipes,
    Inspect,
}

#[derive(Resource, Reflect, Debug, Default)]
pub enum TomeFocused {
    #[default]
    None,
    Item(Entity),
    People(Entity),
    Recipes(AssetId<RecipeDef>),
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIRoot;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UITomeRoot;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UITomeLeftPageRoot;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UITomeRightPageRoot;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIEntryList;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIEntry;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIEntryDetails;

fn spawn_ui_root(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Root"),
        UIRoot,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
    ));
}

fn spawn_tome_root(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root: Single<Entity, With<UIRoot>>,
) {
    commands.spawn((
        Name::new("Book"),
        ChildOf(*ui_root),
        UITomeRoot,
        Node {
            position_type: PositionType::Relative,
            ..default()
        },
        children![
            (widgets::tabs(), observe(on_tab_selection)),
            widgets::page_left(&asset_server),
            widgets::page_right(&asset_server),
        ],
    ));
}

fn on_tab_selection(
    value_change: On<ValueChange<Entity>>,
    q_tabs: Query<&TomeTab, With<RadioButton>>,
    mut next_tab: ResMut<NextState<TomeTab>>,
) {
    if let Ok(tab) = q_tabs.get(value_change.value) {
        next_tab.set(*tab);
    }
}

fn refresh_tab_style(
    q_tabs: Query<(Entity, &TomeTab, Has<Checked>), With<RadioButton>>,
    current_tab: Res<State<TomeTab>>,
    mut commands: Commands,
) {
    for (entity, tab, is_checked) in q_tabs {
        if *current_tab == *tab {
            commands.entity(entity).insert(Checked);
        } else if is_checked {
            commands.entity(entity).remove::<Checked>();
        }
    }
}

fn toggle_tome_open(state: Res<State<TomeOpen>>, mut next_state: ResMut<NextState<TomeOpen>>) {
    next_state.set(TomeOpen(!state.0));
}

fn set_tome_visibility(
    state: Res<State<TomeOpen>>,
    mut tome_node: Single<&mut Node, With<UITomeRoot>>,
) {
    tome_node.display = if state.0 {
        Display::Flex
    } else {
        Display::None
    };
}

fn update_tab_color(q_tabs: Query<(&mut BackgroundColor, Has<Checked>), With<TomeTab>>) {
    for (mut background_color, checked) in q_tabs {
        background_color.0 = if checked {
            TAB_COLOR_CHECKED
        } else {
            TAB_COLOR_DEFAULT
        };
    }
}
