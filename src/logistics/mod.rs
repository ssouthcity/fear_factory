use bevy::prelude::*;

mod conveyor;
mod io;

pub use io::{InputFilter, ResourceInput, ResourceOutput};

pub fn plugin(app: &mut App) {
    app.add_plugins((conveyor::plugin, io::plugin));
}
