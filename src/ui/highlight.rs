use bevy::prelude::*;

use crate::{
    build::Building,
    input::{InputMode, Selection},
};

const DEFAULT_HIGHLIGHT_COLOR: Color = Color::hsl(60.0, 1.0, 0.5);
const DISMANTLE_HIGHLIGHT_COLOR: Color = Color::hsl(0.0, 1.0, 0.5);

pub fn plugin(app: &mut App) {
    app.register_type::<HighlightColor>();
    app.register_type::<Highlightable>();

    app.init_resource::<HighlightColor>();

    app.add_systems(
        Update,
        (
            on_input_mode_changed.run_if(state_changed::<InputMode>),
            highlight,
        )
            .chain(),
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct HighlightColor(Color);

impl Default for HighlightColor {
    fn default() -> Self {
        Self(DEFAULT_HIGHLIGHT_COLOR)
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Highlightable;

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

fn on_input_mode_changed(
    mut highlight_color: ResMut<HighlightColor>,
    input_mode: Res<State<InputMode>>,
) {
    highlight_color.0 = match input_mode.get() {
        InputMode::Normal => DEFAULT_HIGHLIGHT_COLOR,
        InputMode::Dismantle => DISMANTLE_HIGHLIGHT_COLOR,
    };
}
