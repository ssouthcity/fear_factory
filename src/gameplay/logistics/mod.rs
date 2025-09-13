use bevy::prelude::*;

pub mod intersection;
pub mod path;
pub mod porter;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((intersection::plugin, path::plugin, porter::plugin));
}
