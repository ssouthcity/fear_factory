use std::collections::HashSet;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::gameplay::{FactorySystems, world::tilemap::coord::Coord};

pub const DEMOLISH_BUTTON: KeyCode = KeyCode::KeyF;
pub const DEMOLISH_CANCEL_BUTTON: KeyCode = KeyCode::Escape;
pub const DEMOLISH_DURATION_SECS: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DemolishSelection>();
    app.init_resource::<DemolishTimer>();

    app.add_message::<Demolished>();

    app.add_systems(
        FixedUpdate,
        (
            add_to_selection.run_if(on_message::<Pointer<Over>>),
            remove_from_selection.run_if(on_message::<Pointer<Out>>),
            clear_selection.run_if(input_just_pressed(DEMOLISH_CANCEL_BUTTON)),
        )
            .in_set(FactorySystems::Demolish),
    );

    app.add_systems(
        FixedUpdate,
        (
            tick_demolish_timer,
            highlight_demolition,
            demolish_selection.run_if(demolish_timer_finished),
        )
            .chain()
            .in_set(FactorySystems::Demolish),
    );
}

#[derive(Message, Reflect, Debug)]
pub struct Demolished {
    pub entity: Entity,
    pub coord: Coord,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Pickable)]
pub struct Demolishable;

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct DemolishSelection(HashSet<Entity>);

#[derive(Resource, Reflect, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub struct DemolishTimer(Timer);

impl Default for DemolishTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(DEMOLISH_DURATION_SECS, TimerMode::Once))
    }
}

fn tick_demolish_timer(
    mut timer: ResMut<DemolishTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keys.just_released(DEMOLISH_BUTTON) {
        timer.reset();
    }

    if keys.pressed(DEMOLISH_BUTTON) {
        timer.tick(time.delta());
    }
}

fn demolish_timer_finished(timer: Res<DemolishTimer>) -> bool {
    timer.just_finished()
}

fn add_to_selection(
    mut pointer_overs: MessageReader<Pointer<Over>>,
    mut selection: ResMut<DemolishSelection>,
    demolishables: Query<Entity, With<Demolishable>>,
) {
    for pointer_over in pointer_overs.read() {
        if !demolishables.contains(pointer_over.entity) {
            continue;
        }

        selection.insert(pointer_over.entity);
    }
}

fn remove_from_selection(
    mut pointer_outs: MessageReader<Pointer<Out>>,
    mut selection: ResMut<DemolishSelection>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for pointer_out in pointer_outs.read() {
        if keys.pressed(KeyCode::ShiftLeft) {
            continue;
        }

        selection.remove(&pointer_out.entity);
    }
}

fn clear_selection(mut selection: ResMut<DemolishSelection>) {
    selection.clear();
}

fn highlight_demolition(
    timer: Res<DemolishTimer>,
    selection: Res<DemolishSelection>,
    mut sprites: Query<&mut Sprite, With<Demolishable>>,
) {
    let inverse_fraction = 1.0 - timer.fraction();

    let hue = 60.0 * inverse_fraction;

    for entity in selection.iter() {
        if let Ok(mut sprite) = sprites.get_mut(*entity) {
            sprite.color = Color::hsl(hue, 1.0, 0.5);
        }
    }
}

fn demolish_selection(
    mut selection: ResMut<DemolishSelection>,
    mut commands: Commands,
    mut demolitions: MessageWriter<Demolished>,
    coords: Query<&Coord>,
) {
    for demolishable in selection.drain() {
        commands.entity(demolishable).despawn();

        if let Ok(coord) = coords.get(demolishable) {
            demolitions.write(Demolished {
                entity: demolishable,
                coord: Coord(coord.0),
            });
        }
    }
}
