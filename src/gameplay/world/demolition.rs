use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    gameplay::{FactorySystems, world::tilemap::coord::Coord},
    input::input_map::{Action, InputActions, action_just_pressed},
};

pub const DEMOLISH_DURATION_SECS: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DemolishSelection>();
    app.init_resource::<DemolishTimer>();

    app.add_message::<Demolished>();

    app.add_observer(add_to_selection);
    app.add_observer(remove_from_selection);

    app.add_systems(
        Update,
        (
            clear_selection.run_if(action_just_pressed(Action::Dismiss)),
            tick_demolish_timer,
        )
            .chain(),
    );

    app.add_systems(
        FixedUpdate,
        (
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
    time: Res<Time>,
    input_actions: Res<InputActions>,
) {
    if input_actions.pressed.contains(&Action::Demolish) {
        timer.tick(time.delta());
    } else {
        timer.reset();
    }
}

fn demolish_timer_finished(timer: Res<DemolishTimer>) -> bool {
    timer.is_finished()
}

fn add_to_selection(
    pointer_over: On<Pointer<Over>>,
    demolishables: Query<Entity, With<Demolishable>>,
    mut selection: ResMut<DemolishSelection>,
) {
    if demolishables.contains(pointer_over.entity) {
        selection.insert(pointer_over.entity);
    }
}

fn remove_from_selection(
    pointer_out: On<Pointer<Out>>,
    input_actions: Res<InputActions>,
    mut selection: ResMut<DemolishSelection>,
) {
    if input_actions.pressed.contains(&Action::MultiSelect) {
        return;
    }

    selection.remove(&pointer_out.entity);
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
