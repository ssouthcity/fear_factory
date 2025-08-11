use bevy::prelude::*;

mod highlight;
mod hotbar;
mod inspect;
mod interactable;
mod y_sort;

pub use hotbar::{HotbarItemDeselected, HotbarItemSelected, HotbarSelection};
pub use inspect::Inspect;
pub use interactable::{Interact, Interactable};
pub use y_sort::YSort;

const HIGHLIGHT_COLOR: Color = Color::hsl(60.0, 1.0, 0.5);

pub fn plugin(app: &mut App) {
    app.add_plugins((
        highlight::plugin,
        hotbar::plugin,
        inspect::plugin,
        interactable::plugin,
        y_sort::plugin,
    ));
}
