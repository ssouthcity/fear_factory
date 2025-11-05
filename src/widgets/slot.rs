use bevy::{prelude::*, ui_widgets::observe};

use crate::{screens::Screen, widgets::item::stack_icon};

const SLOTTED_OBJECT_Z_INDEX: i32 = 10;
const HELD_OBJECT_Z_INDEX: i32 = 15;
const HOVERED_SLOT_LIGHTEN_FACTOR: f32 = 0.05;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, sync_slot_child.run_if(in_state(Screen::Gameplay)));
}

pub fn slot_container() -> impl Bundle {
    (
        Name::new("Slot"),
        Node {
            width: px(64.0),
            height: px(64.0),
            margin: px(4.0).all(),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::hsl(188.0, 0.94, 0.06)),
        Pickable::default(),
        observe(
            |pointer_over: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(pointer_over.entity) {
                    color.0 = color.0.lighter(HOVERED_SLOT_LIGHTEN_FACTOR);
                }
            },
        ),
        observe(
            |pointer_out: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(pointer_out.entity) {
                    color.0 = color.0.darker(HOVERED_SLOT_LIGHTEN_FACTOR);
                }
            },
        ),
        observe(on_slot_drag_and_drop),
    )
}

pub fn slotted_stack(slot: Entity, stack: Entity) -> impl Bundle {
    (
        Name::new("Slotted Item"),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
        GlobalZIndex(SLOTTED_OBJECT_Z_INDEX),
        InSlot(slot),
        ChildOf(slot),
        Pickable {
            is_hoverable: true,
            should_block_lower: false,
        },
        children![stack_icon(stack)],
        observe(
            |pointer_drag_start: On<Pointer<DragStart>>, mut query: Query<&mut GlobalZIndex>| {
                if let Ok(mut z_index) = query.get_mut(pointer_drag_start.entity) {
                    z_index.0 = HELD_OBJECT_Z_INDEX;
                }
            },
        ),
        observe(
            |pointer_drag: On<Pointer<Drag>>, mut query: Query<&mut UiTransform>| {
                if let Ok(mut transform) = query.get_mut(pointer_drag.entity) {
                    transform.translation.x = px(pointer_drag.distance.x);
                    transform.translation.y = px(pointer_drag.distance.y);
                }
            },
        ),
        observe(
            |pointer_drag_end: On<Pointer<DragEnd>>,
             mut query: Query<(&mut UiTransform, &mut GlobalZIndex)>| {
                if let Ok((mut transform, mut z_index)) = query.get_mut(pointer_drag_end.entity) {
                    transform.translation = Val2::default();
                    z_index.0 = SLOTTED_OBJECT_Z_INDEX;
                }
            },
        ),
    )
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

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[relationship_target(relationship = InSlot, linked_spawn)]
pub struct SlotOccupant(Entity);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[relationship(relationship_target = SlotOccupant)]
pub struct InSlot(pub Entity);

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
