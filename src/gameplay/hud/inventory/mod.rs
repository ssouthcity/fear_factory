use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioButton, RadioGroup, ValueChange, observe},
};

pub mod items;
pub mod people;

pub const TAB_COLOR_DEFAULT: Color = Color::hsl(300.0, 0.25, 0.5);
pub const TAB_COLOR_CHECKED: Color = Color::hsl(160.0, 0.25, 0.5);

pub const ENTRY_COLOR_DEFAULT: Color = Color::NONE;
pub const ENTRY_COLOR_CHECKED: Color = Color::hsla(0.0, 0.0, 0.0, 0.1);

pub const PAGE_WIDTH: f32 = 512.0;
pub const PAGE_HEIGHT: f32 = PAGE_WIDTH * 1.6;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((items::plugin, people::plugin));

    app.add_systems(Startup, spawn_ui_root);

    app.add_systems(
        Update,
        (
            toggle_inventory_ui.run_if(input_just_pressed(KeyCode::KeyT)),
            update_tab_color,
            update_entry_color,
        ),
    );
}

#[derive(Component, Reflect, Debug, Default, PartialEq, Eq)]
#[reflect(Component)]
pub enum UIInventoryTab {
    #[default]
    Items,
    People,
    Recipes,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIRoot;

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

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIInventory;

fn toggle_inventory_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_menu: Option<Single<Entity, With<UIInventory>>>,
    ui_root: Single<Entity, With<UIRoot>>,
) {
    if let Some(menu) = existing_menu {
        commands.entity(*menu).despawn();
        return;
    }

    commands.spawn((
        Name::new("Book"),
        ChildOf(*ui_root),
        UIInventory,
        Node::default(),
        children![(
            Node {
                position_type: PositionType::Relative,
                ..default()
            },
            children![
                tabs(),
                entry_list(&asset_server),
                entry_details(&asset_server)
            ],
        )],
    ));
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
            (tab("Items", UIInventoryTab::Items), Checked),
            tab("People", UIInventoryTab::People),
            tab("Recipes", UIInventoryTab::Recipes),
        ],
    )
}

fn tab(name: &'static str, tab: UIInventoryTab) -> impl Bundle {
    (
        Name::new(name),
        Node::default(),
        BackgroundColor(TAB_COLOR_DEFAULT),
        RadioButton,
        tab,
        children![Text::new(name),],
    )
}

fn update_tab_color(q_tabs: Query<(&mut BackgroundColor, Has<Checked>), With<UIInventoryTab>>) {
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
    q_checked: Query<Entity, (With<RadioButton>, With<UIInventoryTab>, With<Checked>)>,
    mut commands: Commands,
) {
    for radio in q_checked {
        commands.entity(radio).remove::<Checked>();
    }

    commands.entity(value_change.value).insert(Checked);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIEntryList;

fn entry_list(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Entry List"),
        UIEntryList,
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(8.0),
            width: px(PAGE_WIDTH),
            height: px(PAGE_HEIGHT),
            padding: px(PAGE_WIDTH / 8.0).all(),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        ImageNode {
            image: asset_server.load("sprites/hud/tome_left.png"),
            ..default()
        },
        RadioGroup,
        observe(on_entry_selection),
    )
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIEntry;

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

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIEntryDetails;

fn entry_details(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Entry Details"),
        UIEntryDetails,
        Node {
            width: px(PAGE_WIDTH),
            height: px(PAGE_HEIGHT),
            padding: percent(10.0).all(),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        ImageNode {
            image: asset_server.load("sprites/hud/tome_right.png"),
            ..default()
        },
    )
}
