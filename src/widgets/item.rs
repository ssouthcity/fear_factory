use bevy::prelude::*;

use crate::{
    gameplay::{
        FactorySystems,
        item::{Item, assets::ItemDef},
    },
    widgets::tooltip::{HideTooltip, ShowTooltip},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_item_icon_added);

    app.add_systems(Update, update_item_icons.in_set(FactorySystems::UI));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ItemIcon;

pub fn item_icon(handle: Handle<ItemDef>) -> impl Bundle {
    (
        Name::new("Item Icon"),
        ItemIcon,
        Item(handle),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::default(),
        Pickable {
            is_hoverable: true,
            should_block_lower: false,
        },
    )
}

#[allow(clippy::type_complexity)]
fn update_item_icons(
    query: Query<(&Item, &mut ImageNode), (With<ItemIcon>, Changed<Item>)>,
    item_defs: Res<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
) {
    for (item, mut image_node) in query {
        let Some(item_def) = item_defs.get(&item.0) else {
            continue;
        };

        image_node.image = asset_server.load(&item_def.sprite);
    }
}

fn on_item_icon_added(add: On<Add, ItemIcon>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .observe(
            |pointer_over: On<Pointer<Over>>,
             item_icon_query: Query<&Item>,
             items: Res<Assets<ItemDef>>,
             mut commands: Commands| {
                let Ok(item_handle) = item_icon_query.get(pointer_over.entity) else {
                    return;
                };

                let Some(item_def) = items.get(&item_handle.0) else {
                    return;
                };

                commands.trigger(ShowTooltip(item_def.name.clone()));
            },
        )
        .observe(|_pointer_out: On<Pointer<Out>>, mut commands: Commands| {
            commands.trigger(HideTooltip);
        });
}
