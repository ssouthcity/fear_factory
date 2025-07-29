use bevy::prelude::*;

mod hotbar;

pub use hotbar::HotbarSelection;

pub fn plugin(app: &mut App) {
    app.add_plugins(hotbar::plugin);
}
