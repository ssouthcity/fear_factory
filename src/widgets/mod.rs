use bevy::prelude::*;

pub mod person_badge;
pub mod recipe_plate;
pub mod resource_plate;
pub mod tooltip;

pub use person_badge::person_badge;
pub use recipe_plate::recipe_plate;
pub use resource_plate::resource_plate;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        person_badge::plugin,
        recipe_plate::plugin,
        resource_plate::plugin,
        tooltip::plugin,
    ));
}
