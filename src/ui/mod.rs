use bevy::prelude::*;

mod highlight;
mod hotbar;

pub use highlight::Highlightable;
pub use hotbar::HotbarSelection;

pub fn plugin(app: &mut App) {
    app.add_plugins((highlight::plugin, hotbar::plugin));
}
