use bevy::prelude::*;

pub mod assets;
pub mod inventory;

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin,));
}
