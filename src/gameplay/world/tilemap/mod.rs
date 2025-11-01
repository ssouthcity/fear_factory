use bevy::prelude::*;

use crate::{
    gameplay::world::{
        construction::ValidPlacement,
        tilemap::coord::{Coord, translation_to_coord},
    },
    input::cursor::CursorPosition,
};

pub mod chunk;
pub mod coord;

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
pub const TILE_SIZE: Vec2 = Vec2 { x: 512.0, y: 256.0 };
pub const TILE_OFFSET: Vec2 = Vec2 { x: 256.0, y: 128.0 };

pub fn plugin(app: &mut App) {
    app.add_plugins((chunk::plugin, coord::plugin));

    app.add_message::<TileClicked>();

    app.add_observer(on_click);
}

#[derive(Message, Reflect)]
pub struct TileClicked(pub Coord);

fn on_click(
    _click: On<Pointer<Click>>,
    cursor_position: Res<CursorPosition>,
    mut tile_clicks: MessageWriter<TileClicked>,
    valid_placement: Res<ValidPlacement>,
) {
    if valid_placement.0 {
        let coord = translation_to_coord(&cursor_position.0);
        tile_clicks.write(TileClicked(coord));
    }
}
