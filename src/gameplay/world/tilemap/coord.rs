use bevy::prelude::*;

use crate::gameplay::world::tilemap::TILE_OFFSET;

pub fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, translate_coord_to_transform);
}

#[derive(Component, Reflect, Debug, Default, Deref, DerefMut, PartialEq, Eq)]
#[reflect(Component)]
pub struct Coord(pub IVec2);

pub const TILE_MATRIX: Mat2 = Mat2::from_cols(
    Vec2::new(TILE_OFFSET.x * 0.5, -(TILE_OFFSET.y * 0.5)),
    Vec2::new(TILE_OFFSET.x * 0.5, TILE_OFFSET.y * 0.5),
);

pub fn translation_to_coord(translation: &Vec2) -> Coord {
    let ivec = (TILE_MATRIX.inverse() * translation).round().as_ivec2();

    Coord(ivec)
}

pub fn coord_to_translation(coord: &Coord) -> Vec2 {
    TILE_MATRIX * coord.0.as_vec2()
}

fn translate_coord_to_transform(coord_query: Query<(&mut Transform, &Coord), Changed<Coord>>) {
    for (mut transform, coord) in coord_query {
        transform.translation = coord_to_translation(coord).extend(transform.translation.z);
    }
}
