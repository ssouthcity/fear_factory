use bevy::prelude::*;

pub mod assets;
pub mod compendium;
pub mod stack;

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, compendium::plugin, stack::plugin));
}
