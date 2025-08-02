use bevy::prelude::*;

mod highlight;
mod hotbar;

pub use highlight::Highlightable;
pub use hotbar::HotbarSelection;

use crate::FactorySystems;

pub fn plugin(app: &mut App) {
    app.add_plugins((highlight::plugin, hotbar::plugin));

    app.add_systems(Update, y_sort.in_set(FactorySystems::UI));
}

fn y_sort(query: Query<&mut Transform, With<Sprite>>) {
    for mut transform in query {
        transform.translation.z = 1.0 - transform.translation.y.atan() / std::f32::consts::PI + 0.5;
    }
}
