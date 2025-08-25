use bevy::prelude::*;

use crate::simulation::FactorySystems;

pub fn plugin(app: &mut App) {
    app.register_type::<YSort>();

    app.add_systems(Update, y_sort.in_set(FactorySystems::UI));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct YSort(pub f32);

impl Default for YSort {
    fn default() -> Self {
        Self(1.0)
    }
}

fn y_sort(query: Query<(&mut Transform, &YSort)>) {
    for (mut transform, y_sort) in query {
        let atan_mapping = 1.0 - transform.translation.y.atan() / std::f32::consts::PI + 0.5;

        transform.translation.z = y_sort.0 * atan_mapping;
    }
}
