use std::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};

use crate::{FactorySystems, machine::power::Powered};

pub fn plugin(app: &mut App) {
    app.register_type::<BeginWork>();
    app.register_type::<WorkCompleted>();

    app.register_type::<Frequency>();
    app.register_type::<FrequencyTimer>();

    app.add_event::<BeginWork>();
    app.add_event::<WorkCompleted>();

    app.add_systems(FixedUpdate, (tick_frequency_timers, update_progress_bars));

    app.add_systems(
        Update,
        (add_working_tag, remove_working_tag).in_set(FactorySystems::Work),
    );
}

#[derive(Event, Reflect)]
pub struct BeginWork(pub Entity);

#[derive(Event, Reflect)]
pub struct WorkCompleted(pub Entity);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Working;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(immutable, on_insert = on_frequency_insert)]
pub struct Frequency(pub Duration);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FrequencyTimer(pub Timer);

fn on_frequency_insert(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let Some(frequency) = world.get::<Frequency>(entity) else {
        unreachable!("frequency is guaranteed to be Some");
    };

    let duration = frequency.0;

    world
        .commands()
        .entity(entity)
        .insert(FrequencyTimer(Timer::new(duration, TimerMode::Repeating)));

    world.commands().spawn((
        Name::new("Progress Bar"),
        ChildOf(entity),
        Transform::from_xyz(0.0, 48.0, 0.0),
        Sprite::from_color(Color::BLACK, Vec2::new(64.0, 16.0)),
        children![(
            Name::new("Progress Bar Fill"),
            FrequencyProgressOf(entity),
            Transform::from_xyz(-32.0, 0.0, 0.0),
            Sprite {
                color: Color::linear_rgb(0.0, 0.8, 0.1),
                rect: Some(Rect::new(0.0, 0.0, 64.0, 16.0)),
                anchor: Anchor::CenterLeft,
                ..default()
            },
        )],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FrequencyProgressOf(Entity);

fn add_working_tag(
    mut events: EventReader<BeginWork>,
    mut commands: Commands,
    mut frequency_timers: Query<&mut FrequencyTimer>,
) {
    for event in events.read() {
        commands.entity(event.0).insert(Working);

        if let Ok(mut frequency) = frequency_timers.get_mut(event.0) {
            frequency.0.reset();
        }
    }
}

fn tick_frequency_timers(
    time: Res<Time>,
    frequencies: Query<(Entity, &mut FrequencyTimer), (With<Powered>, With<Working>)>,
    mut work_completed_events: EventWriter<WorkCompleted>,
) {
    for (entity, mut frequency) in frequencies {
        frequency.0.tick(time.delta());

        if frequency.0.just_finished() {
            work_completed_events.write(WorkCompleted(entity));
        }
    }
}

fn remove_working_tag(mut events: EventReader<WorkCompleted>, mut commands: Commands) {
    for event in events.read() {
        commands.entity(event.0).remove::<Working>();
    }
}

fn update_progress_bars(
    frequency_timers: Query<&FrequencyTimer>,
    progress_bars: Query<(&mut Sprite, &FrequencyProgressOf)>,
) {
    for (mut sprite, progress_bar_of) in progress_bars {
        let Ok(frequency_timer) = frequency_timers.get(progress_bar_of.0) else {
            continue;
        };

        let Some(rect) = sprite.rect else {
            continue;
        };

        sprite.custom_size = Some(Vec2::new(
            rect.width() * frequency_timer.0.fraction(),
            rect.height(),
        ));
    }
}
