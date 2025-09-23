use bevy::prelude::*;

use crate::gameplay::{FactorySystems, sprite_sort::ZIndexSprite};

pub fn plugin(app: &mut App) {
    app.register_type::<Coord>();

    app.add_systems(Update, sync_z_index.in_set(FactorySystems::UI));
}

#[derive(Component, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Coord(pub UVec3);

fn sync_z_index(query: Query<(&Coord, &mut ZIndexSprite)>) {
    for (coord, mut z_index) in query {
        z_index.0 = coord.z;
    }
}
