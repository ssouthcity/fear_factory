use bevy::prelude::*;

pub mod item;
pub mod slot;
pub mod tooltip;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((item::plugin, slot::plugin, tooltip::plugin));
}

pub fn container() -> impl Bundle {
    Node {
        width: percent(100.0),
        height: percent(100.0),
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
