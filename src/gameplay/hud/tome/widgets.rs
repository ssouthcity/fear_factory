use bevy::{
    prelude::*,
    ui_widgets::{RadioButton, RadioGroup},
};

use crate::gameplay::hud::tome::{PAGE_HEIGHT, PAGE_WIDTH, UIEntryList};

pub fn page_left(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: px(PAGE_WIDTH),
            height: px(PAGE_HEIGHT),
            padding: percent(6.0).all(),
            ..default()
        },
        ImageNode {
            image: asset_server.load("sprites/hud/tome_left.png"),
            ..default()
        },
    )
}

pub fn page_right(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: px(PAGE_WIDTH),
            height: px(PAGE_HEIGHT),
            padding: percent(6.0).all(),
            ..default()
        },
        ImageNode {
            image: asset_server.load("sprites/hud/tome_right.png"),
            ..default()
        },
    )
}

pub fn list_page() -> impl Bundle {
    (
        UIEntryList,
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(8.0),
            width: percent(100.0),
            height: percent(100.0),
            padding: percent(5.0).all(),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        RadioGroup,
    )
}

pub fn list_entry() -> impl Bundle {
    (
        Node {
            column_gap: px(4.0),
            ..default()
        },
        RadioButton,
    )
}
