use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;

use crate::{
    screens::Screen,
    simulation::item::{Item, ItemAssets, ItemDef},
    ui::widgets,
};

const SLOTTED_OBJECT_Z_INDEX: i32 = 10;
const HELD_OBJECT_Z_INDEX: i32 = 15;
const HOVERED_SLOT_LIGHTEN_FACTOR: f32 = 0.05;

pub fn plugin(app: &mut App) {
    app.register_type::<AddedToSlot>();
    app.register_type::<RemovedFromSlot>();

    app.register_type::<Slot>();
    app.register_type::<SlotOccupant>();
    app.register_type::<InSlot>();

    app.add_observer(on_add_slot);
    app.add_observer(on_add_slot_occupant);

    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(Update, sync_slot_child.run_if(in_state(Screen::Gameplay)));
}

#[derive(Event, Reflect)]
pub struct AddedToSlot(pub Entity);

#[derive(Event, Reflect)]
pub struct RemovedFromSlot(pub Entity);

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

fn on_add_slot(trigger: Trigger<OnAdd, Slot>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(
            |trigger: Trigger<Pointer<Over>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(trigger.target) {
                    color.0 = color.0.lighter(HOVERED_SLOT_LIGHTEN_FACTOR);
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>, mut query: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = query.get_mut(trigger.target) {
                    color.0 = color.0.darker(HOVERED_SLOT_LIGHTEN_FACTOR);
                }
            },
        )
        .observe(on_slot_drag_and_drop);
}

fn on_add_slot_occupant(trigger: Trigger<OnAdd, InSlot>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(
            |trigger: Trigger<Pointer<DragStart>>,
             mut query: Query<(&mut Node, &mut GlobalZIndex)>| {
                if let Ok((mut node, mut z_index)) = query.get_mut(trigger.target) {
                    node.position_type = PositionType::Absolute;
                    z_index.0 = HELD_OBJECT_Z_INDEX;
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
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    item_query: Query<&InSlot>,
    slot_query: Query<&SlotOccupant>,
    // names: Query<&Name>,
) {
    let destination_slot = trigger.target();
    let source_item = trigger.dropped;

    let Ok(source_slot) = item_query.get(source_item).map(|slot| slot.0) else {
        return;
    };

    if source_slot == destination_slot {
        return;
    }

    // info!(
    //     "moving {} to {}",
    //     names
    //         .get(source_item)
    //         .map(|a| a.to_string())
    //         .unwrap_or("unknown".into()),
    //     names
    //         .get(destination_slot)
    //         .map(|a| a.to_string())
    //         .unwrap_or("unknown".into()),
    // );

    let destination_item = slot_query
        .get(destination_slot)
        .map(|slotted_item| slotted_item.0);

    if let Ok(destination_item) = destination_item {
        commands.entity(destination_item).remove::<InSlot>();
    }

    commands
        .entity(source_item)
        .insert(InSlot(destination_slot));

    commands
        .entity(source_slot)
        .trigger(RemovedFromSlot(source_item));

    if let Ok(destination_item) = destination_item {
        commands
            .entity(destination_item)
            .insert(InSlot(source_slot));

        commands
            .entity(destination_slot)
            .trigger(RemovedFromSlot(destination_item));

        commands
            .entity(source_slot)
            .trigger(AddedToSlot(destination_item));
    }

    commands
        .entity(destination_slot)
        .trigger(AddedToSlot(source_item));
}

fn setup(
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    mut item_defs: ResMut<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
) {
    let items = item_defs
        .iter()
        .map(|(a, b)| (a, b.clone()))
        .collect::<Vec<_>>();

    let mut items_iter = items.iter();

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

        if let Some((asset_id, item_def)) = items_iter.next() {
            let id = commands
                .spawn((
                    Name::new(item_def.name.clone()),
                    InSlot(slot_id),
                    Item(item_defs.get_strong_handle(*asset_id).unwrap()),
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        ..default()
                    },
                ))
                .id();

            if let Some(sprite_path) = &item_def.sprite {
                commands.entity(id).insert(ImageNode {
                    image: asset_server.load(sprite_path.clone()),
                    ..default()
                });
            } else {
                commands.entity(id).insert((
                    ImageNode::default(),
                    AseSlice {
                        aseprite: item_assets.aseprite.clone(),
                        name: item_def.id.clone(),
                    },
                ));
            }
        }
    }
}
