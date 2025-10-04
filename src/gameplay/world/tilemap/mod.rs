use bevy::prelude::*;

pub mod chunk;
pub mod coord;
pub mod map;

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
pub const TILE_SIZE: Vec2 = Vec2 { x: 256.0, y: 128.0 };
pub const TILE_OFFSET: Vec2 = Vec2 { x: 128.0, y: 64.0 };

pub fn plugin(app: &mut App) {
    app.add_plugins((chunk::plugin, coord::plugin, map::plugin));
}
