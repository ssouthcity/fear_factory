use bevy::prelude::*;

pub mod hotbar;
pub mod inspect;
pub mod item_slot;

pub fn plugin(app: &mut App) {
    app.add_plugins((hotbar::plugin, inspect::plugin, item_slot::plugin));
}
