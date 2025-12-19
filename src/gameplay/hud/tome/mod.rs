use bevy::{
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioButton, RadioGroup, ValueChange, observe},
};

use crate::{
    gameplay::recipe::assets::RecipeDef,
    input::input_map::{Action, action_just_pressed},
    screens::Screen,
};

pub mod items;
pub mod people;
pub mod widgets;

pub const TAB_COLOR_DEFAULT: Color = Color::hsl(300.0, 0.25, 0.5);
pub const TAB_COLOR_CHECKED: Color = Color::hsl(160.0, 0.25, 0.5);

pub const ENTRY_COLOR_DEFAULT: Color = Color::NONE;
pub const ENTRY_COLOR_CHECKED: Color = Color::hsla(0.0, 0.0, 0.0, 0.1);

pub const PAGE_WIDTH: f32 = 512.0;
pub const PAGE_HEIGHT: f32 = PAGE_WIDTH * 1.6;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((items::plugin, people::plugin));

    app.add_sub_state::<TomeOpen>();
    app.add_sub_state::<TomeTab>();
    app.init_resource::<TomeDetails>();

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

    app.add_systems(Update, (update_tab_color, update_entry_color));
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
}

#[derive(Resource, Reflect, Debug, Default)]
pub enum TomeDetails {
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
            tabs(),
            (widgets::page_left(&asset_server), children![entry_list()],),
            (
                widgets::page_right(&asset_server),
                children![entry_details()],
            )
        ],
    ));
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

fn tabs() -> impl Bundle {
    (
        Name::new("Tabs"),
        Node {
            position_type: PositionType::Absolute,
            top: px(64.0),
            right: px(PAGE_WIDTH * 2.0 - 32.0),
            flex_direction: FlexDirection::Column,
            row_gap: px(8.0),
            ..default()
        },
        ZIndex(10),
        RadioGroup,
        observe(on_tab_selection),
        children![
            (tab("Items", TomeTab::Items), Checked),
            tab("People", TomeTab::People),
            tab("Recipes", TomeTab::Recipes),
        ],
    )
}

fn tab(name: &'static str, tab: TomeTab) -> impl Bundle {
    (
        Name::new(name),
        Node::default(),
        BackgroundColor(TAB_COLOR_DEFAULT),
        RadioButton,
        tab,
        children![Text::new(name),],
    )
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

fn update_entry_color(q_entries: Query<(&mut BackgroundColor, Has<Checked>), With<UIEntry>>) {
    for (mut background_color, checked) in q_entries {
        background_color.0 = if checked {
            ENTRY_COLOR_CHECKED
        } else {
            ENTRY_COLOR_DEFAULT
        };
    }
}

fn on_tab_selection(
    value_change: On<ValueChange<Entity>>,
    q_checked: Query<Entity, (With<RadioButton>, With<TomeTab>, With<Checked>)>,
    mut commands: Commands,
) {
    for radio in q_checked {
        commands.entity(radio).remove::<Checked>();
    }

    commands.entity(value_change.value).insert(Checked);
}

fn entry_list() -> impl Bundle {
    (
        Name::new("Entry List"),
        UIEntryList,
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(8.0),
            width: percent(100.0),
            height: percent(100.0),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        RadioGroup,
        observe(on_entry_selection),
    )
}

fn on_entry_selection(
    value_change: On<ValueChange<Entity>>,
    q_checked: Query<Entity, (With<RadioButton>, With<UIEntry>, With<Checked>)>,
    mut commands: Commands,
) {
    for entry in q_checked {
        commands.entity(entry).remove::<Checked>();
    }

    commands.entity(value_change.value).insert(Checked);
}

fn entry_details() -> impl Bundle {
    (
        Name::new("Entry Details"),
        UIEntryDetails,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            overflow: Overflow::scroll_y(),
            ..default()
        },
    )
}
