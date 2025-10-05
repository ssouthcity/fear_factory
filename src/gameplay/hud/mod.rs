use bevy::prelude::*;

pub mod hotbar;
pub mod inspect;

pub fn plugin(app: &mut App) {
    app.add_plugins((hotbar::plugin, inspect::plugin));
}
