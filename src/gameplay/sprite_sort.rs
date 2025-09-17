use bevy::prelude::*;

use crate::gameplay::FactorySystems;

pub fn plugin(app: &mut App) {
    app.register_type::<YSortSprite>();
    app.register_type::<ZIndexSprite>();

    app.add_systems(Update, sort_sprites.in_set(FactorySystems::UI));
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(ZIndexSprite)]
pub struct YSortSprite;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ZIndexSprite(pub u32);

fn sort_sprites(query: Query<(&mut Transform, &ZIndexSprite, Option<&YSortSprite>)>) {
    for (mut transform, z_index, y_sort) in query {
        let mut z_coordinate = z_index.0 as f32;

        if y_sort.is_some() {
            let atan_mapping = 1.0 - transform.translation.y.atan() / std::f32::consts::PI + 0.5;
            z_coordinate += atan_mapping;
        }

        transform.translation.z = z_coordinate;
    }
}
