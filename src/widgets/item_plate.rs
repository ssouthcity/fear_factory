use bevy::{prelude::*, ui_widgets::RadioButton};

use crate::gameplay::item::{assets::ItemDef, stack::Stack};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, refresh_item_plates);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemPlate(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemPortrait;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemName;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemQuantity;

pub fn item_plate(item: Entity) -> impl Bundle {
    (
        Node {
            column_gap: px(4.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        RadioButton,
        ItemPlate(item),
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
    q_item_plates: Query<(Entity, &ItemPlate)>,
    q_item_stacks: Query<&Stack>,
    item_defs: Res<Assets<ItemDef>>,
    children: Query<&Children>,
    mut q_item_plate_components: ParamSet<(
        Query<&mut ImageNode, With<ItemPortrait>>,
        Query<&mut Text, With<ItemName>>,
        Query<&mut Text, With<ItemQuantity>>,
    )>,
    asset_server: Res<AssetServer>,
) {
    for (item_plate, ItemPlate(stack_entity)) in q_item_plates {
        let Ok(stack) = q_item_stacks.get(*stack_entity) else {
            continue;
        };

        let Some(item_def) = item_defs.get(&stack.item) else {
            return;
        };

        for child in children.iter_descendants(item_plate) {
            if let Ok(mut image) = q_item_plate_components.p0().get_mut(child) {
                image.image = asset_server.load(item_def.sprite.clone());
            }

            if let Ok(mut text) = q_item_plate_components.p1().get_mut(child) {
                text.0 = item_def.name.clone();
            }

            if let Ok(mut text) = q_item_plate_components.p2().get_mut(child) {
                text.0 = stack.quantity.to_string();
            }
        }
    }
}
