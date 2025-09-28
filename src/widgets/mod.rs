use bevy::prelude::*;

use crate::gameplay::hud::item_slot::Slot;

pub mod item;
pub mod tooltip;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((item::plugin, tooltip::plugin));
}

pub fn container() -> impl Bundle {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn slot() -> impl Bundle {
    (
        Name::new("Slot"),
        Slot,
        Node {
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            margin: UiRect::all(Val::Px(4.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::hsl(188.0, 0.94, 0.06)),
    )
}
