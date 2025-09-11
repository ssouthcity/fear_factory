use bevy::prelude::*;

pub mod indexing;
pub mod loaders;
pub mod manifest;
pub mod tracking;

pub fn plugin(app: &mut App) {
    app.add_plugins((tracking::plugin,));
}
