use bevy::prelude::*;

pub mod intersection;
pub mod path;
pub mod pathfinding;
pub mod porter;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        intersection::plugin,
        path::plugin,
        pathfinding::plugin,
        porter::plugin,
    ));
}
