use bevy::{prelude::*, ui::Checked, ui_widgets::RadioButton};

use crate::gameplay::{
    hud::inventory::{InInventory, UIEntry, UIEntryDetails, UIEntryList, UIInventoryTab},
    item::{assets::ItemDef, stack::Stack},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (fill_item_inventory, update_item_entry).chain());

    app.add_systems(Update, (fill_item_details, update_item_details).chain());
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ItemHandle(Handle<ItemDef>);

fn item_entry(handle: Handle<ItemDef>) -> impl Bundle {
    (
        Node {
            column_gap: px(4.0),
            ..default()
        },
        UIEntry,
        RadioButton,
        ItemHandle(handle),
    )
}

fn fill_item_inventory(
    q_tab: Single<&UIInventoryTab, Changed<Checked>>,
    q_entry_list: Single<Entity, With<UIEntryList>>,
    q_entry_details: Single<Entity, With<UIEntryDetails>>,
    q_items: Query<&Stack, With<InInventory>>,
    mut commands: Commands,
) {
    commands.entity(*q_entry_list).despawn_children();
    commands.entity(*q_entry_details).despawn_children();

    if **q_tab != UIInventoryTab::Items {
        return;
    }

    for stack in q_items {
        commands.spawn((item_entry(stack.item.clone()), ChildOf(*q_entry_list)));
    }
}

fn update_item_entry(
    q_entries: Query<(Entity, &ItemHandle), (With<UIEntry>, Changed<ItemHandle>)>,
    item_defs: Res<Assets<ItemDef>>,
    mut commands: Commands,
) {
    for (entry, item_handle) in q_entries {
        let Some(item_def) = item_defs.get(&item_handle.0) else {
            continue;
        };

        commands.entity(entry).despawn_children();

        commands.entity(entry).insert(children![(
            Text::new(item_def.name.clone()),
            TextFont::default().with_font_size(32.0),
        )]);
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UIItemDetails;

fn item_details(handle: Handle<ItemDef>) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(8.0),
            ..default()
        },
        UIItemDetails,
        ItemHandle(handle),
    )
}

fn fill_item_details(
    q_entry: Single<&ItemHandle, (With<UIEntry>, Changed<Checked>)>,
    q_entry_details: Single<Entity, With<UIEntryDetails>>,
    mut commands: Commands,
) {
    commands.entity(*q_entry_details).despawn_children();

    commands.spawn((item_details(q_entry.0.clone()), ChildOf(*q_entry_details)));
}

fn update_item_details(
    q_entry_details: Query<(Entity, &ItemHandle), (With<UIItemDetails>, Changed<ItemHandle>)>,
    item_defs: Res<Assets<ItemDef>>,
    mut commands: Commands,
) {
    for (details, item_handle) in q_entry_details {
        let Some(item_def) = item_defs.get(&item_handle.0) else {
            continue;
        };

        commands.entity(details).despawn_children();

        commands.entity(details).insert(children![
            Text::new(format!("Name: {}", item_def.name)),
            Text::new(format!("Description: {}", item_def.description)),
            Text::new(format!("Taxonomy: {:#?}", item_def.taxonomy)),
        ]);
    }
}
