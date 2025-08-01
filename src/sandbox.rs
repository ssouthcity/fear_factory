use bevy::prelude::*;

use crate::{build::QueueSpawnBuilding, ui::HotbarSelection};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_sandbox);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Sandbox;

fn spawn_sandbox(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Sandbox"),
            Sandbox::default(),
            Sprite::from_color(Color::hsl(100.0, 0.5, 0.64), Vec2::splat(1600.0)),
            Pickable::default(),
        ))
        .observe(queue_spawn_building);
}

fn queue_spawn_building(
    trigger: Trigger<Pointer<Click>>,
    mut events: EventWriter<QueueSpawnBuilding>,
    selected_buildable: Res<HotbarSelection>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let Some(cursor_position) = trigger.event().hit.position else {
        return;
    };

    events.write(QueueSpawnBuilding {
        buildable: selected_buildable.0,
        position: cursor_position.truncate(),
    });
}
