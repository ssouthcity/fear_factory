use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::gameplay::{
    FactorySystems,
    world::tilemap::chunk::{Chunk, Layers},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Coord>();

    app.add_systems(
        Update,
        translate_coord_to_transform.in_set(FactorySystems::UI),
    );
}

#[derive(Component, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Coord(pub UVec2);

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Self(UVec2 { x, y })
    }
}

#[allow(clippy::type_complexity)]
fn translate_coord_to_transform(
    coord_query: Query<(&mut Transform, &Coord), Changed<Coord>>,
    chunk_query: Single<&Layers, With<Chunk>>,
    tile_storage_query: Query<&TileStorage>,
    tilemap_query: Query<
        (
            &Transform,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &TilemapAnchor,
        ),
        Without<Coord>,
    >,
) {
    for (mut transform, coord) in coord_query {
        let tile_pos = TilePos::new(coord.x, coord.y);

        let Some(layer) = chunk_query.iter().find(|layer| {
            tile_storage_query
                .get(*layer)
                .is_ok_and(|storage| storage.get(&tile_pos).is_none())
        }) else {
            continue;
        };

        let Ok((tilemap_transform, map_size, grid_size, tile_size, map_type, anchor)) =
            tilemap_query.get(layer)
        else {
            continue;
        };

        let tile_translation = tile_pos
            .center_in_world(map_size, grid_size, tile_size, map_type, anchor)
            .extend(0.0);

        let z = transform.translation.z;
        transform.translation = tile_translation + tilemap_transform.translation;
        transform.translation.z = z;
    }
}
