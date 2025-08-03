use bevy::prelude::*;

use crate::{
    build::Building,
    dismantle::{DismantleTimer, Selection},
};

const DEFAULT_HIGHLIGHT_COLOR: Color = Color::hsl(60.0, 1.0, 0.5);
// const DISMANTLE_HIGHLIGHT_COLOR: Color = Color::hsl(0.0, 1.0, 0.5);

pub fn plugin(app: &mut App) {
    app.register_type::<HighlightColor>();

    app.init_resource::<HighlightColor>();

    app.add_systems(Update, (calculate_dismantle_color, highlight).chain());
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct HighlightColor(Color);

impl Default for HighlightColor {
    fn default() -> Self {
        Self(DEFAULT_HIGHLIGHT_COLOR)
    }
}

fn highlight(
    building_sprites: Query<(Entity, &mut Sprite), With<Building>>,
    highlight_color: Res<HighlightColor>,
    selection: Res<Selection>,
) {
    for (building, mut sprite) in building_sprites {
        let color = if selection.contains(&building) {
            highlight_color.0
        } else {
            Default::default()
        };

        sprite.color = color;
    }
}

fn calculate_dismantle_color(
    timer: Res<DismantleTimer>,
    mut highlight_color: ResMut<HighlightColor>,
) {
    let inverse_fraction = 1.0 - timer.fraction();

    let hue = 60.0 * inverse_fraction;

    highlight_color.0.set_hue(hue);
}
