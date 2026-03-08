use bevy::prelude::*;
use bevy::ui_widgets::observe;

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
struct ItemEntry(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ItemSpriteFor(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ItemQuantityFor(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct DropZoneOverlay;

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
                    position_type: PositionType::Relative,
                    display: Display::None,
                    padding: px(16).all(),
                    border: px(4).all(),
                    border_radius: BorderRadius::all(px(8)),
                    row_gap: px(8),
                    ..default()
                },
                BackgroundColor(Color::hsl(30.0, 0.60, 0.63)),
                BorderColor::all(Color::hsl(24.0, 0.74, 0.51)),
                droppable(),
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
                    (
                        DropZoneOverlay,
                        Node {
                            display: Display::None,
                            position_type: PositionType::Absolute,
                            width: percent(100),
                            height: percent(100),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                        ZIndex(999),
                        GlobalZIndex(0),
                        children![(
                            Text::new("Drop item to deposit"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
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

        commands.spawn((ui_item_stack(*item), ChildOf(*item_ui_grid)));
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

fn draggable() -> impl Bundle {
    (
        GlobalZIndex::default(),
        observe(
            |over: On<Pointer<Over>>, mut query: Query<&mut UiTransform>| {
                if let Ok(mut ui_transform) = query.get_mut(over.entity) {
                    ui_transform.scale = Vec2::splat(1.1);
                }
            },
        ),
        observe(
            |out: On<Pointer<Out>>, mut query: Query<&mut UiTransform>| {
                if let Ok(mut ui_transform) = query.get_mut(out.entity) {
                    ui_transform.scale = Vec2::splat(1.0);
                }
            },
        ),
        observe(
            |drag_start: On<Pointer<DragStart>>, mut query: Query<&mut GlobalZIndex>| {
                if let Ok(mut global_z_index) = query.get_mut(drag_start.entity) {
                    global_z_index.0 = 1;
                }
            },
        ),
        observe(
            |drag: On<Pointer<Drag>>, mut query: Query<&mut UiTransform>| {
                if let Ok(mut ui_transform) = query.get_mut(drag.entity) {
                    ui_transform.translation = Val2::px(drag.distance.x, drag.distance.y);
                }
            },
        ),
        observe(
            |drag_end: On<Pointer<DragEnd>>,
             mut query: Query<(&mut GlobalZIndex, &mut UiTransform)>| {
                if let Ok((mut global_z_index, mut ui_transform)) = query.get_mut(drag_end.entity) {
                    global_z_index.0 = 0;
                    ui_transform.translation = Val2::ZERO;
                }
            },
        ),
    )
}

fn droppable() -> impl Bundle {
    (
        observe(
            |drag_enter: On<Pointer<DragEnter>>,
             item_entries: Query<(), With<ItemEntry>>,
             mut overlay_query: Query<&mut Node, With<DropZoneOverlay>>| {
                if !item_entries.contains(drag_enter.dragged) {
                    return;
                }

                if let Ok(mut overlay) = overlay_query.single_mut() {
                    overlay.display = Display::Flex;
                }
            },
        ),
        observe(
            |drag_leave: On<Pointer<DragLeave>>,
             item_entries: Query<(), With<ItemEntry>>,
             mut overlay_query: Query<&mut Node, With<DropZoneOverlay>>| {
                if !item_entries.contains(drag_leave.dragged) {
                    return;
                }

                if let Ok(mut overlay) = overlay_query.single_mut() {
                    overlay.display = Display::None;
                }
            },
        ),
        observe(
            |drag_drop: On<Pointer<DragDrop>>,
             item_entries: Query<(), With<ItemEntry>>,
             mut overlay_query: Query<&mut Node, With<DropZoneOverlay>>| {
                if !item_entries.contains(drag_drop.dropped) {
                    return;
                }

                if let Ok(mut overlay) = overlay_query.single_mut() {
                    overlay.display = Display::None;
                }
            },
        ),
    )
}

fn ui_item_stack(item_stack: Entity) -> impl Bundle {
    (
        Name::new("UI Item Stack"),
        ItemEntry(item_stack),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            border: px(4).all(),
            border_radius: BorderRadius::all(px(8)),
            ..default()
        },
        BackgroundColor(Color::hsl(16.0, 0.42, 0.4)),
        BorderColor::all(Color::hsl(9.0, 0.35, 0.3)),
        draggable(),
        Pickable {
            should_block_lower: false,
            ..default()
        },
        children![
            (
                Name::new("Item Sprite"),
                Node {
                    width: px(64),
                    height: px(64),
                    border: UiRect::bottom(px(4)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::hsl(9.0, 0.35, 0.3)),
                Pickable::IGNORE,
                children![(
                    Node {
                        max_width: percent(100),
                        max_height: percent(100),
                        ..default()
                    },
                    ImageNode::default(),
                    ItemSpriteFor(item_stack),
                    Pickable::IGNORE,
                )]
            ),
            (
                Name::new("Item Quantity"),
                Text::default(),
                ItemQuantityFor(item_stack),
                Pickable::IGNORE,
            ),
        ],
    )
}
