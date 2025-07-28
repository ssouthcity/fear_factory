use bevy::prelude::*;

mod hotbar;
mod input_mode;

pub use hotbar::HotbarSelection;

pub fn plugin(app: &mut App) {
    app.add_plugins((hotbar::plugin, input_mode::plugin));
}
