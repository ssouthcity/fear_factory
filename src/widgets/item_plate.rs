use bevy::{prelude::*, ui_widgets::RadioButton};

use crate::gameplay::item::{assets::ItemDef, inventory::Inventory};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, refresh_item_plates);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemPlate(Entity, AssetId<ItemDef>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemPortrait;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemName;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemQuantity;

pub fn item_plate(owner: Entity, item_id: AssetId<ItemDef>) -> impl Bundle {
    (
        Node {
            column_gap: px(4.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        RadioButton,
        ItemPlate(owner, item_id),
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

fn refresh_item_plates(
    item_plates: Query<(Entity, &ItemPlate)>,
    inventories: Query<&Inventory>,
    item_defs: Res<Assets<ItemDef>>,
    children: Query<&Children>,
    mut item_plate_components: ParamSet<(
        Query<&mut ImageNode, With<ItemPortrait>>,
        Query<&mut Text, With<ItemName>>,
        Query<&mut Text, With<ItemQuantity>>,
    )>,
    asset_server: Res<AssetServer>,
) {
    for (item_plate, ItemPlate(owner, item_id)) in item_plates {
        let Ok(inventory) = inventories.get(*owner) else {
            continue;
        };

        let quantity = inventory.items.get(item_id).unwrap_or(&0);

        let Some(item_def) = item_defs.get(*item_id) else {
            return;
        };

        for child in children.iter_descendants(item_plate) {
            if let Ok(mut image) = item_plate_components.p0().get_mut(child) {
                image.image = asset_server.load(item_def.sprite.clone());
            }

            if let Ok(mut text) = item_plate_components.p1().get_mut(child) {
                text.0 = item_def.name.clone();
            }

            if let Ok(mut text) = item_plate_components.p2().get_mut(child) {
                text.0 = quantity.to_string();
            }
        }
    }
}
