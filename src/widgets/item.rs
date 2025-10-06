use bevy::{prelude::*, ui_widgets::observe};

use crate::{
    gameplay::{
        FactorySystems,
        item::{
            assets::ItemDef,
            stack::{Full, Stack},
        },
    },
    widgets::tooltip::{HideTooltip, ShowTooltip},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_item_icons.in_set(FactorySystems::UI));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct StackIcon(pub Entity);

pub fn stack_icon(stack: Entity) -> impl Bundle {
    (
        Name::new("Item Icon"),
        StackIcon(stack),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            position_type: PositionType::Relative,
            ..default()
        },
        Pickable {
            is_hoverable: true,
            should_block_lower: false,
        },
        children![
            (
                Node {
                    width: percent(100.0),
                    height: percent(100.0),
                    ..default()
                },
                ImageNode::default(),
                Pickable::IGNORE,
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::ZERO,
                    bottom: Val::ZERO,
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                TextColor(Color::BLACK),
                Text::default(),
                Pickable::IGNORE,
            ),
        ],
        observe(
            |pointer_over: On<Pointer<Over>>,
             stack_icon_query: Query<&StackIcon>,
             stack_query: Query<&Stack>,
             items: Res<Assets<ItemDef>>,
             mut commands: Commands| {
                let Ok(stack_icon) = stack_icon_query.get(pointer_over.entity) else {
                    return;
                };

                let Ok(stack) = stack_query.get(stack_icon.0) else {
                    return;
                };

                let Some(item_def) = items.get(&stack.item) else {
                    return;
                };

                commands.trigger(ShowTooltip(item_def.name.clone()));
            },
        ),
        observe(|_pointer_out: On<Pointer<Out>>, mut commands: Commands| {
            commands.trigger(HideTooltip);
        }),
    )
}

#[allow(clippy::type_complexity)]
fn update_item_icons(
    stack_icon_query: Query<(&StackIcon, &Children)>,
    stack_query: Query<(&Stack, Option<&Full>)>,
    mut image_node_query: Query<&mut ImageNode>,
    mut quantity_text_query: Query<(&mut Text, &mut TextColor)>,
    item_defs: Res<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
) {
    for (StackIcon(stack), children) in stack_icon_query {
        let Ok((stack, full)) = stack_query.get(*stack) else {
            continue;
        };

        let Some(item_def) = item_defs.get(&stack.item) else {
            continue;
        };

        let mut children_iter = children.iter();

        if let Ok(mut image_node) = image_node_query.get_mut(children_iter.next().unwrap()) {
            image_node.image = asset_server.load(item_def.sprite.to_owned());
        }

        if let Ok((mut text, mut text_color)) =
            quantity_text_query.get_mut(children_iter.next().unwrap())
        {
            text.0 = stack.quantity.to_string();
            text_color.0 = if full.is_some() {
                Color::hsl(60.0, 1.0, 0.5)
            } else {
                Color::BLACK
            };
        }
    }
}
