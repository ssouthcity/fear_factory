use bevy::prelude::*;

pub mod cursor;
pub mod input_map;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((cursor::plugin, input_map::plugin));
}
