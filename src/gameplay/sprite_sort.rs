use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, sort_sprites);
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
        let mut z_coordinate = z_index.0 as f32 * 10.0;

        if y_sort.is_some() {
            z_coordinate -= transform.translation.y * 0.001;
        }

        transform.translation.z = z_coordinate;
    }
}
