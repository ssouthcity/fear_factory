use bevy::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        item::assets::ItemDef,
        storage::{ResourceID, ResourceStorage},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, refresh_resource_plates);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ResourcePlate(pub Entity, pub ResourceID);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemPortrait;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemName;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemQuantity;

pub fn resource_plate(owner: Entity, resource: ResourceID) -> impl Bundle {
    (
        Node {
            column_gap: px(4.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        ResourcePlate(owner, resource),
        children![
            (
                Node {
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        ItemPortrait,
                        ImageNode::default(),
                        Node {
                            width: px(64.0),
                            height: px(64.0),
                            ..default()
                        }
                    ),
                    (
                        ItemName,
                        Text::default(),
                        TextFont::default().with_font_size(32.0),
                    ),
                ],
            ),
            (
                Node {
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    ItemQuantity,
                    Text::default(),
                    TextFont::default().with_font_size(24.0),
                ),],
            ),
        ],
    )
}

fn refresh_resource_plates(
    q_item_plates: Query<(Entity, &ResourcePlate)>,
    q_resource_storage: Query<&ResourceStorage>,
    item_defs: Res<Assets<ItemDef>>,
    item_index: Res<IndexMap<ItemDef>>,
    children: Query<&Children>,
    mut q_item_plate_components: ParamSet<(
        Query<&mut ImageNode, With<ItemPortrait>>,
        Query<&mut Text, With<ItemName>>,
        Query<&mut Text, With<ItemQuantity>>,
    )>,
    asset_server: Res<AssetServer>,
) {
    for (item_plate, ResourcePlate(parent, resource)) in q_item_plates {
        let Ok(storage) = q_resource_storage.get(*parent) else {
            continue;
        };

        let Some(item_def) = item_index
            .get(&resource.0)
            .and_then(|id| item_defs.get(*id))
        else {
            continue;
        };

        for child in children.iter_descendants(item_plate) {
            if let Ok(mut image) = q_item_plate_components.p0().get_mut(child) {
                image.image = asset_server.load(item_def.sprite.clone());
            }

            if let Ok(mut text) = q_item_plate_components.p1().get_mut(child) {
                text.0 = item_def.name.clone();
            }

            if let Ok(mut text) = q_item_plate_components.p2().get_mut(child) {
                text.0 = storage.resources.get(resource).unwrap_or(&0).to_string();
            }
        }
    }
}
