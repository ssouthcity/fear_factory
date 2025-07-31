use bevy::prelude::*;

mod conveyor;
mod io;
mod item;

pub use io::{ResourceInput, ResourceOutput};
pub use item::{ItemCollection, ItemID};

pub fn plugin(app: &mut App) {
    app.add_plugins((conveyor::plugin, io::plugin, item::plugin));
}
