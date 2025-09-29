use bevy::prelude::*;

use crate::{
    gameplay::{
        FactorySystems,
        item::{Full, Item, Quantity, assets::ItemDef},
    },
    widgets::tooltip::{HideTooltip, ShowTooltip},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ItemIcon>();
    app.register_type::<StackIconOf>();

    app.add_observer(on_item_icon_added);
    app.add_observer(on_stack_icon_added);

    app.add_systems(
        Update,
        (update_item_icons, update_stack_icons).in_set(FactorySystems::UI),
    );
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

fn on_item_icon_added(trigger: Trigger<OnAdd, (ItemIcon, StackIconOf)>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             item_icon_query: Query<&Item>,
             items: Res<Assets<ItemDef>>,
             mut commands: Commands| {
                let Ok(item_handle) = item_icon_query.get(trigger.target) else {
                    return;
                };

                let Some(item_def) = items.get(&item_handle.0) else {
                    return;
                };

                commands.trigger(ShowTooltip(item_def.name.clone()));
            },
        )
        .observe(|_trigger: Trigger<Pointer<Out>>, mut commands: Commands| {
            commands.trigger(HideTooltip);
        });
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(ImageNode)]
pub struct StackIconOf(pub Entity);

fn update_stack_icons(
    stack_icon_query: Query<(&StackIconOf, &Children)>,
    mut image_query: Query<&mut ImageNode>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
    stack_query: Query<(&Item, &Quantity, Option<&Full>)>,
    item_defs: Res<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
) {
    for (StackIconOf(entity), children) in stack_icon_query {
        let Ok((item, quantity, full)) = stack_query.get(*entity) else {
            continue;
        };

        let Some(item_def) = item_defs.get(&item.0) else {
            continue;
        };

        let mut children_iter = children.iter();

        if let Ok(mut image_node) = image_query.get_mut(children_iter.next().unwrap()) {
            image_node.image = asset_server.load(&item_def.sprite);
        }

        if let Ok((mut text, mut text_color)) = text_query.get_mut(children_iter.next().unwrap()) {
            text.0 = quantity.0.to_string();

            text_color.0 = if full.is_some() {
                Color::hsl(33.0, 0.65, 0.75)
            } else {
                Color::BLACK
            };
        }
    }
}

pub fn stack_icon(stack: Entity) -> impl Bundle {
    (
        Name::new("Item Icon"),
        StackIconOf(stack),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
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
            )
        ],
    )
}

fn on_stack_icon_added(trigger: Trigger<OnAdd, StackIconOf>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             stack_icon_query: Query<&StackIconOf>,
             stack_query: Query<&Item>,
             items: Res<Assets<ItemDef>>,
             mut commands: Commands| {
                let Ok(StackIconOf(stack_entity)) = stack_icon_query.get(trigger.target) else {
                    return;
                };

                let Ok(Item(handle)) = stack_query.get(*stack_entity) else {
                    return;
                };

                let Some(item_def) = items.get(handle) else {
                    return;
                };

                commands.trigger(ShowTooltip(item_def.name.clone()));
            },
        )
        .observe(|_trigger: Trigger<Pointer<Out>>, mut commands: Commands| {
            commands.trigger(HideTooltip);
        });
}
