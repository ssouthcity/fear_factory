use bevy::{prelude::*, ui_widgets::RadioButton};

use crate::gameplay::inventory::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, refresh_item_plates);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemPlate(Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemPortrait;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemName;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemQuantity;

pub fn item_plate(entity: Entity) -> impl Bundle {
    (
        Node {
            column_gap: px(4.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        RadioButton,
        ItemPlate(entity),
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
    item_defs: Res<Assets<ItemDef>>,
    stacks: Query<&ItemStack>,
    mut images: ResMut<Assets<Image>>,
    children: Query<&Children>,
    mut item_plate_components: ParamSet<(
        Query<&mut ImageNode, With<ItemPortrait>>,
        Query<&mut Text, With<ItemName>>,
        Query<&mut Text, With<ItemQuantity>>,
    )>,
) {
    for (item_plate, ItemPlate(entity)) in item_plates {
        let Ok(stack) = stacks.get(*entity) else {
            continue;
        };

        let Some(item_def) = item_defs.get(&stack.item) else {
            return;
        };

        for child in children.iter_descendants(item_plate) {
            if let Ok(mut image_node) = item_plate_components.p0().get_mut(child)
                && let Some(image) = images.get_strong_handle(item_def.sprite)
            {
                image_node.image = image;
            }

            if let Ok(mut text) = item_plate_components.p1().get_mut(child) {
                text.0 = item_def.name.clone();
            }

            if let Ok(mut text) = item_plate_components.p2().get_mut(child) {
                text.0 = stack.quantity.to_string();
            }
        }
    }
}
