use bevy::prelude::*;

use crate::{
    FactorySystems,
    power::socket::{PowerSockets, PowerSocketsLinked},
    sandbox::Sandbox,
};

const POWER_LINE_COLOR: Color = Color::BLACK;

pub fn plugin(app: &mut App) {
    app.register_type::<PowerLine>();

    app.add_systems(
        Update,
        (
            create_power_line.in_set(FactorySystems::Build),
            garbage_clean_power_lines.in_set(FactorySystems::GarbageClean),
            draw_power_lines.in_set(FactorySystems::UI),
        ),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerLine(pub Entity, pub Entity);

fn create_power_line(
    mut events: EventReader<PowerSocketsLinked>,
    mut commands: Commands,
    sandbox: Single<Entity, With<Sandbox>>,
) {
    for event in events.read() {
        commands.spawn((
            Name::new("Power Line"),
            ChildOf(*sandbox),
            PowerLine(event.0, event.1),
        ));
    }
}

fn garbage_clean_power_lines(
    power_lines: Query<(Entity, &PowerLine)>,
    entities: Query<Entity, With<PowerSockets>>,
    mut commands: Commands,
) {
    for (entity, PowerLine(from, to)) in power_lines {
        if !entities.contains(*from) || !entities.contains(*to) {
            commands.entity(entity).despawn();
        }
    }
}

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
