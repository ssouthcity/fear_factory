use bevy::{picking::hover::PickingInteraction, prelude::*};

use crate::gameplay::FactorySystems;

const HIGHLIGHT_COLOR: Color = Color::hsl(60.0, 1.0, 0.5);

pub fn plugin(app: &mut App) {
    app.init_resource::<HighlightColor>();

    app.add_systems(Update, highlight_pickable.before(FactorySystems::Demolish));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct HighlightColor(Color);

impl Default for HighlightColor {
    fn default() -> Self {
        Self(HIGHLIGHT_COLOR)
    }
}

fn highlight_pickable(query: Query<(&mut Sprite, &PickingInteraction)>) {
    for (mut sprite, picking_interaction) in query {
        if *picking_interaction != PickingInteraction::None {
            sprite.color = HIGHLIGHT_COLOR;
        } else {
            sprite.color = default();
        }
    }
}
