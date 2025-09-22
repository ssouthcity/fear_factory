use bevy::prelude::*;

pub mod cursor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(cursor::plugin);
}
