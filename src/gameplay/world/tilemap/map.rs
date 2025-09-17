use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    gameplay::{
        FactorySystems,
        world::tilemap::{
            TILE_OFFSET,
            chunk::{Chunk, Layers},
        },
    },
    input::cursor::CursorPosition,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HoveredTile>();
    app.init_resource::<HoveredTile>();

    app.register_type::<TerrainClick>();
    app.add_event::<TerrainClick>();

    app.add_systems(
        Update,
        propogate_world_click
            .run_if(on_event::<Pointer<Click>>)
            .in_set(FactorySystems::Input),
    );

    app.add_systems(
        Update,
        (mark_highlighted_tile, highlight_hovered_tile)
            .chain()
            .in_set(FactorySystems::UI),
    );
}

#[derive(Event, Reflect, Debug)]
pub struct TerrainClick {
    pub entity: Entity,
    pub position: Vec2,
}

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
    chunk_query: Single<&Layers, With<Chunk>>,
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
    for layer in chunk_query.iter().rev() {
        let Ok((map_size, grid_size, tile_size, map_type, tile_storage, map_transform, anchor)) =
            tilemap_query.get(layer)
        else {
            continue;
        };

        let cursor_in_map_pos = {
            let cursor_pos = Vec4::from((cursor_position.0, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
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
            continue;
        };

        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            hovered_tile.0 = tile_entity;
            return;
        }
    }

    hovered_tile.0 = Entity::PLACEHOLDER;
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
    tile_query: Query<(&TilePos, &TilemapId)>,
    tilemap_query: Query<(
        &Transform,
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    mut events: EventWriter<TerrainClick>,
) {
    let Ok((tile_pos, tilemap_id)) = tile_query.get(hovered_tile.0) else {
        return;
    };

    let Ok((transform, map_size, grid_size, tile_size, map_type, anchor)) =
        tilemap_query.get(tilemap_id.0)
    else {
        return;
    };

    let translation = tile_pos
        .center_in_world(map_size, grid_size, tile_size, map_type, anchor)
        .extend(0.0);

    events.write(TerrainClick {
        entity: Entity::PLACEHOLDER,
        position: (translation + transform.translation + Vec3::Y * TILE_OFFSET.y).xy(),
    });
}
