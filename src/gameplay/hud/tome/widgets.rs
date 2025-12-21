use bevy::{
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioButton, RadioGroup},
};

use crate::gameplay::hud::tome::{TomeTab, UIEntryList, UITomeLeftPageRoot, UITomeRightPageRoot};

pub const TAB_COLOR_DEFAULT: Color = Color::hsl(300.0, 0.25, 0.5);
pub const TAB_COLOR_CHECKED: Color = Color::hsl(160.0, 0.25, 0.5);

pub const ENTRY_COLOR_DEFAULT: Color = Color::NONE;
pub const ENTRY_COLOR_CHECKED: Color = Color::hsla(0.0, 0.0, 0.0, 0.1);

pub const PAGE_WIDTH: f32 = 512.0;
pub const PAGE_HEIGHT: f32 = PAGE_WIDTH * 1.6;

pub fn tabs() -> impl Bundle {
    (
        Name::new("Tabs"),
        Node {
            position_type: PositionType::Absolute,
            top: px(64.0),
            right: px(PAGE_WIDTH * 2.0 - 32.0),
            flex_direction: FlexDirection::Column,
            row_gap: px(8.0),
            ..default()
        },
        ZIndex(10),
        RadioGroup,
        children![
            (tab("Items", TomeTab::Items), Checked),
            tab("People", TomeTab::People),
            tab("Recipes", TomeTab::Recipes),
        ],
    )
}

pub fn tab(name: &'static str, tab: TomeTab) -> impl Bundle {
    (
        Name::new(name),
        Node::default(),
        BackgroundColor(TAB_COLOR_DEFAULT),
        RadioButton,
        tab,
        children![Text::new(name),],
    )
}

pub fn page_left(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Left Page"),
        UITomeLeftPageRoot,
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
        Name::new("Right Page"),
        UITomeRightPageRoot,
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
