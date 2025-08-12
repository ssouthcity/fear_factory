use bevy::{picking::hover::PickingInteraction, prelude::*};

use crate::{
    dismantle::{DismantleTimer, Selection},
    sandbox::{Building, Sandbox},
};

use super::HIGHLIGHT_COLOR;

pub fn plugin(app: &mut App) {
    app.register_type::<HighlightColor>();

    app.init_resource::<HighlightColor>();

    app.add_systems(
        Update,
        (highlight_pickable, calculate_dismantle_color, highlight).chain(),
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct HighlightColor(Color);

impl Default for HighlightColor {
    fn default() -> Self {
        Self(HIGHLIGHT_COLOR)
    }
}

fn highlight_pickable(query: Query<(&mut Sprite, &PickingInteraction), Without<Sandbox>>) {
    for (mut sprite, picking_interaction) in query {
        if *picking_interaction != PickingInteraction::None {
            sprite.color = HIGHLIGHT_COLOR;
        } else {
            sprite.color = default();
        }
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
