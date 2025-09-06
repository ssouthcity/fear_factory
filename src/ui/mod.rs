use bevy::prelude::*;

mod highlight;
mod hotbar;
mod inspect;
mod interactable;
pub mod widgets;
mod y_sort;

pub use self::{
    hotbar::{HotbarItemDeselected, HotbarItemSelected, HotbarSelection},
    inspect::Inspect,
    interactable::{Interact, Interactable},
    y_sort::YSort,
};

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
