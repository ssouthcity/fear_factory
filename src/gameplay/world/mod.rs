use bevy::prelude::*;

pub mod construction;
pub mod demolition;
pub mod deposit;
pub mod tilemap;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        construction::plugin,
        demolition::plugin,
        deposit::plugin,
        tilemap::plugin,
    ));
}
