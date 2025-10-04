use bevy::prelude::*;

use crate::{
    gameplay::item::assets::ItemDef,
    screens::Screen,
    widgets::{self, item::item_icon},
};

const SLOTTED_OBJECT_Z_INDEX: i32 = 10;
const HELD_OBJECT_Z_INDEX: i32 = 15;
const HOVERED_SLOT_LIGHTEN_FACTOR: f32 = 0.05;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_slot);
    app.add_observer(on_add_slot_occupant);

    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(Update, sync_slot_child.run_if(in_state(Screen::Gameplay)));
}

#[derive(EntityEvent, Reflect)]
pub struct AddedToSlot {
    pub entity: Entity,
    pub item: Entity,
}

#[derive(EntityEvent, Reflect)]
pub struct RemovedFromSlot {
    pub entity: Entity,
    pub item: Entity,
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
#[require(
    Pickable {
        should_block_lower: false,
        is_hoverable: true,
    },
    GlobalZIndex(SLOTTED_OBJECT_Z_INDEX),
)]
pub struct InSlot(pub Entity);

fn on_add_slot(add: On<Add, Slot>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .observe(
            |pointer_over: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(pointer_over.entity) {
                    color.0 = color.0.lighter(HOVERED_SLOT_LIGHTEN_FACTOR);
                }
            },
        )
        .observe(
            |pointer_out: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(pointer_out.entity) {
                    color.0 = color.0.darker(HOVERED_SLOT_LIGHTEN_FACTOR);
                }
            },
        )
        .observe(on_slot_drag_and_drop);
}

fn on_add_slot_occupant(add: On<Add, InSlot>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .observe(
            |pointer_drag_start: On<Pointer<DragStart>>,
             mut query: Query<(&mut Node, &mut GlobalZIndex)>| {
                if let Ok((mut node, mut z_index)) = query.get_mut(pointer_drag_start.entity) {
                    node.position_type = PositionType::Absolute;
                    z_index.0 = HELD_OBJECT_Z_INDEX;
                }
            },
        )
        .observe(
            |pointer_drag: On<Pointer<Drag>>, mut query: Query<&mut Node>| {
                if let Ok(mut node) = query.get_mut(pointer_drag.entity) {
                    node.left = Val::Px(pointer_drag.distance.x);
                    node.top = Val::Px(pointer_drag.distance.y);
                }
            },
        )
        .observe(
            |pointer_drag_end: On<Pointer<DragEnd>>,
             mut query: Query<(&mut Node, &mut GlobalZIndex)>| {
                if let Ok((mut node, mut z_index)) = query.get_mut(pointer_drag_end.entity) {
                    node.position_type = PositionType::default();
                    node.left = Val::Auto;
                    node.top = Val::Auto;
                    z_index.0 = SLOTTED_OBJECT_Z_INDEX;
                }
            },
        );
}

fn sync_slot_child(query: Query<(Entity, &InSlot), Changed<InSlot>>, mut commands: Commands) {
    for (entity, InSlot(slot)) in query {
        commands.entity(entity).insert(ChildOf(*slot));
    }
}

fn on_slot_drag_and_drop(
    pointer_drag_drop: On<Pointer<DragDrop>>,
    mut commands: Commands,
    item_query: Query<&InSlot>,
    slot_query: Query<&SlotOccupant>,
) {
    let destination_slot = pointer_drag_drop.entity;
    let source_item = pointer_drag_drop.dropped;

    let Ok(source_slot) = item_query.get(source_item).map(|slot| slot.0) else {
        return;
    };

    if source_slot == destination_slot {
        return;
    }

    let destination_item = slot_query
        .get(destination_slot)
        .map(|slotted_item| slotted_item.0);

    if let Ok(destination_item) = destination_item {
        commands.entity(destination_item).remove::<InSlot>();
    }

    commands
        .entity(source_item)
        .insert(InSlot(destination_slot));

    commands.trigger(RemovedFromSlot {
        item: source_item,
        entity: source_slot,
    });

    if let Ok(destination_item) = destination_item {
        commands
            .entity(destination_item)
            .insert(InSlot(source_slot));

        commands.trigger(RemovedFromSlot {
            item: destination_item,
            entity: destination_slot,
        });

        commands.trigger(AddedToSlot {
            item: destination_item,
            entity: source_slot,
        });
    }

    commands.trigger(AddedToSlot {
        item: source_item,
        entity: destination_slot,
    });
}

fn setup(mut commands: Commands, item_defs: Res<Assets<ItemDef>>, asset_server: Res<AssetServer>) {
    let mut items_iter = item_defs.iter();

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
                display: Display::Grid,
                ..default()
            },
            ChildOf(container_id),
            BackgroundColor(Color::hsl(180.0, 1.0, 0.5)),
        ))
        .id();

    for i in 0..10 {
        let grid_cell_id = commands
            .spawn((
                Node {
                    grid_column: GridPlacement::start(i % 5 + 1),
                    grid_row: GridPlacement::start(i / 5 + 1),
                    ..default()
                },
                ChildOf(grid_id),
            ))
            .id();

        let slot_id = commands
            .spawn((widgets::slot(), ChildOf(grid_cell_id)))
            .id();

        if let Some((asset_id, _)) = items_iter.next() {
            commands.spawn((
                InSlot(slot_id),
                ChildOf(slot_id),
                item_icon(asset_server.get_id_handle(asset_id).unwrap()),
            ));
        }
    }
}
