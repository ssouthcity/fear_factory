use bevy::prelude::*;

use crate::FactorySystems;

const POWER_LINE_COLOR: Color = Color::BLACK;

pub fn plugin(app: &mut App) {
    app.register_type::<PowerLine>();

    app.add_systems(Update, draw_power_lines.in_set(FactorySystems::UI));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerLine(pub Entity, pub Entity);

fn draw_power_lines(
    power_lines: Query<&PowerLine>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for PowerLine(from, to) in power_lines {
        let from_position = transforms.get(*from).unwrap().translation.truncate();
        let to_position = transforms.get(*to).unwrap().translation.truncate();

        gizmos.line_2d(from_position, to_position, POWER_LINE_COLOR);
    }
}
