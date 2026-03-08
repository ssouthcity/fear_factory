use bevy::{prelude::*, ui_widgets::observe};

use crate::{
    gameplay::{
        inventory::prelude::{ItemDef, ItemStack},
        player::{ItemAddedToInventory, Player},
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<PlayerInventoryUiState>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_inventory_ui);

    app.add_systems(
        Update,
        (
            on_inventory_hotkey,
            sync_items_to_ui,
            update_images_for_items,
            update_quantity_for_items,
        ),
    );

    app.add_systems(
        Update,
        (update_button_style, update_inventory_ui_visibility)
            .run_if(state_changed::<PlayerInventoryUiState>),
    );
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum PlayerInventoryUiState {
    #[default]
    Closed,
    Open,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct InventoryUiButton;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct InventoryUiContainer;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct InventoryUiGrid;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ItemSpriteFor(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ItemQuantityFor(Entity);

fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            padding: px(16).all(),
            row_gap: px(8),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::End,
            ..default()
        },
        Pickable::IGNORE,
        children![
            (
                Name::new("Inventory UI Icon"),
                InventoryUiButton,
                Node {
                    width: px(64),
                    height: px(64),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(percent(100)),
                    ..default()
                },
                BorderColor::all(Color::BLACK),
                BackgroundColor(Color::BLACK.lighter(0.1)),
                observe(on_inventory_button_click),
                children![(ImageNode::new(asset_server.load("sprites/hud/inventory.png"))),],
            ),
            (
                Name::new("Inventory UI Container"),
                InventoryUiContainer,
                Node {
                    display: Display::None,
                    padding: px(16).all(),
                    border: px(4).all(),
                    border_radius: BorderRadius::all(px(8)),
                    row_gap: px(8),
                    ..default()
                },
                BackgroundColor(Color::hsl(30.0, 0.60, 0.63)),
                BorderColor::all(Color::hsl(24.0, 0.74, 0.51)),
                children![
                    (Text::new("Inventory")),
                    (
                        Name::new("Inventory UI Grid"),
                        InventoryUiGrid,
                        Node {
                            display: Display::Grid,
                            grid_template_columns: vec![RepeatedGridTrack::fr(5, 1.0)],
                            column_gap: px(8),
                            row_gap: px(8),
                            ..default()
                        },
                    ),
                ],
            )
        ],
    ));
}

fn sync_items_to_ui(
    mut items_added_to_inventory: MessageReader<ItemAddedToInventory>,
    player: Single<Entity, With<Player>>,
    item_ui_grid: Single<Entity, With<InventoryUiGrid>>,
    mut commands: Commands,
) {
    for ItemAddedToInventory { item, inventory } in items_added_to_inventory.read() {
        if *inventory != *player {
            continue;
        }

        commands.spawn((
            ChildOf(*item_ui_grid),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                border: px(4).all(),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::hsl(16.0, 0.42, 0.4)),
            BorderColor::all(Color::hsl(9.0, 0.35, 0.3)),
            children![
                (
                    Node {
                        width: px(64),
                        height: px(64),
                        border: UiRect::bottom(px(4)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::all(Color::hsl(9.0, 0.35, 0.3)),
                    children![(
                        Node {
                            max_width: percent(100),
                            max_height: percent(100),
                            ..default()
                        },
                        ImageNode::default(),
                        ItemSpriteFor(*item),
                    )]
                ),
                (Text::default(), ItemQuantityFor(*item),),
            ],
        ));
    }
}

fn on_inventory_hotkey(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<PlayerInventoryUiState>>,
    mut next_state: ResMut<NextState<PlayerInventoryUiState>>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    next_state.set(match state.get() {
        PlayerInventoryUiState::Closed => PlayerInventoryUiState::Open,
        PlayerInventoryUiState::Open => PlayerInventoryUiState::Closed,
    });
}

fn on_inventory_button_click(
    _click: On<Pointer<Click>>,
    state: Res<State<PlayerInventoryUiState>>,
    mut next_state: ResMut<NextState<PlayerInventoryUiState>>,
) {
    next_state.set(match state.get() {
        PlayerInventoryUiState::Closed => PlayerInventoryUiState::Open,
        PlayerInventoryUiState::Open => PlayerInventoryUiState::Closed,
    });
}

fn update_button_style(
    query: Query<&mut BackgroundColor, With<InventoryUiButton>>,
    state: Res<State<PlayerInventoryUiState>>,
) {
    for mut background_color in query {
        let color = match state.get() {
            PlayerInventoryUiState::Closed => Color::BLACK.lighter(0.1),
            PlayerInventoryUiState::Open => Color::BLACK.lighter(0.2),
        };

        background_color.0 = color;
    }
}

fn update_inventory_ui_visibility(
    query: Query<&mut Node, With<InventoryUiContainer>>,
    state: Res<State<PlayerInventoryUiState>>,
) {
    for mut node in query {
        let visibility = match state.get() {
            PlayerInventoryUiState::Closed => Display::None,
            PlayerInventoryUiState::Open => Display::Grid,
        };

        node.display = visibility;
    }
}

fn update_images_for_items(
    query: Query<(&mut ImageNode, &ItemSpriteFor), Changed<ItemSpriteFor>>,
    stacks: Query<&ItemStack>,
    item_definitions: Res<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
) {
    for (mut image_node, item_sprite_for) in query {
        let Ok(stack) = stacks.get(item_sprite_for.0) else {
            continue;
        };

        let Some(item_def) = item_definitions.get(&stack.item) else {
            continue;
        };

        let Some(path) = asset_server.get_path(item_def.sprite) else {
            continue;
        };

        image_node.image = asset_server.load(path);
    }
}

fn update_quantity_for_items(
    query: Query<(&mut Text, &ItemQuantityFor)>,
    stacks: Query<&ItemStack>,
) {
    for (mut text, item_quantity_for) in query {
        let Ok(stack) = stacks.get(item_quantity_for.0) else {
            continue;
        };

        text.0 = stack.quantity.to_string();
    }
}
