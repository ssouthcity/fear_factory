use bevy::prelude::*;

pub mod path;
pub mod pathfinding;
pub mod porter;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((path::plugin, pathfinding::plugin, porter::plugin));
}
