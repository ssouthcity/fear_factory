use bevy::prelude::*;

pub mod assets;
pub mod inventory;
pub mod stack;

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, inventory::plugin, stack::plugin));
}
