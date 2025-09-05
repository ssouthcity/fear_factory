use bevy::prelude::*;
use rand::Rng;

const TILE_SIZE: f32 = 64.0;
const GAP: f32 = 8.0;

pub fn plugin(app: &mut App) {
    app.register_type::<Slot>();
    app.register_type::<SlotOccupant>();
    app.register_type::<InSlot>();

    app.add_observer(on_add_slot);

    app.add_systems(Startup, setup);
    app.add_systems(Update, sync_slot_child);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Pickable)]
pub struct Slot;

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[relationship_target(relationship = InSlot, linked_spawn)]
pub struct SlotOccupant(Entity);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[relationship(relationship_target = SlotOccupant)]
pub struct InSlot(pub Entity);

fn on_add_slot(trigger: Trigger<OnAdd, Slot>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(
            |trigger: Trigger<Pointer<Over>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(trigger.target) {
                    color.0 = color.0.lighter(0.05);
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(trigger.target) {
                    color.0 = color.0.darker(0.05);
                }
            },
        )
        .observe(on_slot_drag_and_drop);
}

fn sync_slot_child(query: Query<(Entity, &InSlot), Changed<InSlot>>, mut commands: Commands) {
    for (entity, InSlot(slot)) in query {
        commands.entity(entity).insert(ChildOf(*slot));
    }
}

fn on_slot_drag_and_drop(
    mut trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    item_query: Query<&InSlot>,
    slot_query: Query<&SlotOccupant>,
) {
    let destination_slot = trigger.target();
    let source_item = trigger.dropped;

    let Ok(source_slot) = item_query.get(source_item).map(|slot| slot.0) else {
        return;
    };

    let destination_item = slot_query
        .get(destination_slot)
        .map(|slotted_item| slotted_item.0);

    if let Ok(destination_item) = destination_item {
        commands.entity(destination_item).remove::<InSlot>();
    }

    commands
        .entity(source_item)
        .insert(InSlot(destination_slot));

    if let Ok(destination_item) = destination_item {
        commands
            .entity(destination_item)
            .insert(InSlot(source_slot));
    }

    trigger.propagate(false);
}

fn setup(mut commands: Commands) {
    let mut rng = rand::rng();

    let container_id = commands
        .spawn((
            Name::new("Inventory"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();

    let grid_id = commands
        .spawn((
            Name::new("Grid"),
            Node {
                padding: UiRect::all(Val::Px(GAP)),
                display: Display::Grid,
                grid_auto_columns: GridTrack::px(TILE_SIZE),
                grid_auto_rows: GridTrack::px(TILE_SIZE),
                column_gap: Val::Px(GAP),
                row_gap: Val::Px(GAP),
                ..default()
            },
            ChildOf(container_id),
            BackgroundColor(Color::hsl(180.0, 1.0, 0.5)),
        ))
        .id();

    for i in 0..10 {
        let slot_id = commands
            .spawn((
                Name::new(format!("Slot {i}")),
                Slot,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    grid_column: GridPlacement::start(i % 5 + 1),
                    grid_row: GridPlacement::start(i / 5 + 1),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ChildOf(grid_id),
                BackgroundColor(Color::BLACK),
            ))
            .id();

        if i % 3 == 0 {
            commands
                .spawn((
                    Name::new(format!("Item {i}")),
                    InSlot(slot_id),
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::hsl(rng.random_range(0.0..360.0), 1.0, 0.5)),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                    GlobalZIndex(10),
                    children![(Text::new(i.to_string()), Pickable::IGNORE,)],
                ))
                .observe(
                    |trigger: Trigger<Pointer<DragStart>>,
                     mut query: Query<(&mut Node, &mut GlobalZIndex)>| {
                        if let Ok((mut node, mut z_index)) = query.get_mut(trigger.target) {
                            node.position_type = PositionType::Absolute;
                            z_index.0 = 20;
                        }
                    },
                )
                .observe(
                    |trigger: Trigger<Pointer<Drag>>, mut query: Query<&mut Node>| {
                        if let Ok(mut node) = query.get_mut(trigger.target) {
                            node.left = Val::Px(trigger.distance.x);
                            node.top = Val::Px(trigger.distance.y);
                        }
                    },
                )
                .observe(
                    |trigger: Trigger<Pointer<DragEnd>>,
                     mut query: Query<(&mut Node, &mut GlobalZIndex)>| {
                        if let Ok((mut node, mut z_index)) = query.get_mut(trigger.target) {
                            node.position_type = PositionType::default();
                            node.left = Val::Auto;
                            node.top = Val::Auto;
                            z_index.0 = 10;
                        }
                    },
                );
        }
    }
}
