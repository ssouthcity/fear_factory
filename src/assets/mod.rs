use bevy::prelude::*;

pub mod indexing;
pub mod loaders;
pub mod manifest;
mod tracking;

pub use self::tracking::{LoadResource, is_finished_loading};

pub fn plugin(app: &mut App) {
    app.add_plugins((tracking::plugin,));
}
