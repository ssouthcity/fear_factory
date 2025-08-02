use bevy::prelude::*;

mod highlight;
mod hotbar;
mod y_sort;

pub use highlight::Highlightable;
pub use hotbar::HotbarSelection;
pub use y_sort::YSort;

pub fn plugin(app: &mut App) {
    app.add_plugins((highlight::plugin, hotbar::plugin, y_sort::plugin));
}
