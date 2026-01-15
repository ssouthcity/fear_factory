use bevy::prelude::*;

pub mod construction;
pub mod demolition;
pub mod tilemap;

pub fn plugin(app: &mut App) {
    app.add_plugins((construction::plugin, demolition::plugin, tilemap::plugin));
}
