use bevy::prelude::*;

pub mod item_plate;
pub mod person_badge;
pub mod recipe_plate;
pub mod tooltip;

pub use item_plate::item_plate;
pub use person_badge::person_badge;
pub use recipe_plate::recipe_plate;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        item_plate::plugin,
        person_badge::plugin,
        recipe_plate::plugin,
        tooltip::plugin,
    ));
}
