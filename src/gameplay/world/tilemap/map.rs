use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    gameplay::{
        FactorySystems,
        world::{
            construction::ValidPlacement,
            tilemap::{chunk::Chunk, coord::Coord},
        },
    },
    input::cursor::CursorPosition,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<HoveredTile>();

    app.add_message::<TileClicked>();

    app.add_systems(
        Update,
        (
            mark_highlighted_tile,
            propogate_world_click.run_if(on_message::<Pointer<Click>>),
        )
            .in_set(FactorySystems::Input),
    );

    app.add_systems(Update, highlight_hovered_tile.in_set(FactorySystems::UI));
}

#[derive(Message, Reflect, Debug)]
pub struct TileClicked(pub Coord);

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct HoveredTile(pub Entity);

impl Default for HoveredTile {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

fn mark_highlighted_tile(
    cursor_position: Res<CursorPosition>,
    mut hovered_tile: ResMut<HoveredTile>,
    chunk_entity: Single<Entity, With<Chunk>>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TileStorage,
        &Transform,
        &TilemapAnchor,
    )>,
) {
    let Ok((map_size, grid_size, tile_size, map_type, tile_storage, map_transform, anchor)) =
        tilemap_query.get(*chunk_entity)
    else {
        return;
    };

    let cursor_in_map_pos = {
        let cursor_pos = Vec4::from((cursor_position.0, 0.0, 1.0));
        let cursor_in_map_pos = map_transform.to_matrix().inverse() * cursor_pos;
        cursor_in_map_pos.xy()
    };

    let Some(tile_pos) = TilePos::from_world_pos(
        &cursor_in_map_pos,
        map_size,
        grid_size,
        tile_size,
        map_type,
        anchor,
    ) else {
        hovered_tile.0 = Entity::PLACEHOLDER;
        return;
    };

    if let Some(tile_entity) = tile_storage.get(&tile_pos) {
        hovered_tile.0 = tile_entity;
    }
}

fn highlight_hovered_tile(
    mut tile_query: Query<&mut TileColor>,
    hovered_tile: Res<HoveredTile>,
    mut previous_tile: Local<HoveredTile>,
) {
    if previous_tile.0 == hovered_tile.0 {
        return;
    }

    if let Ok(mut tile_color) = tile_query.get_mut(previous_tile.0) {
        tile_color.0 = Color::default();
    }

    if let Ok(mut tile_color) = tile_query.get_mut(hovered_tile.0) {
        tile_color.0 = Color::hsl(60.0, 1.0, 0.5);
    }

    previous_tile.0 = hovered_tile.0;
}

fn propogate_world_click(
    hovered_tile: Res<HoveredTile>,
    valid_placement: Res<ValidPlacement>,
    tile_query: Query<&TilePos>,
    mut tile_clicks: MessageWriter<TileClicked>,
) {
    if !valid_placement.0 {
        return;
    }

    let Ok(tile_pos) = tile_query.get(hovered_tile.0) else {
        return;
    };

    tile_clicks.write(TileClicked(Coord::new(tile_pos.x, tile_pos.y)));
}
