use std::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};

use crate::power::fuse_is_broken;

pub fn plugin(app: &mut App) {
    // components
    app.register_type::<Frequency>();
    app.register_type::<FrequencyTimer>();
    // events
    app.register_type::<Work>();

    app.add_systems(
        FixedUpdate,
        (tick_frequency_timers, update_progress_bars)
            .chain()
            .run_if(not(fuse_is_broken)),
    );
}

// Component that determines how often a machine runs
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

    let duration = frequency.0.clone();

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

#[derive(Event, Reflect)]
pub struct Work;

fn tick_frequency_timers(
    mut commands: Commands,
    time: Res<Time>,
    frequencies: Query<(Entity, &mut FrequencyTimer)>,
) {
    for (entity, mut frequency) in frequencies {
        frequency.0.tick(time.delta());

        if frequency.0.just_finished() {
            commands.trigger_targets(Work, entity);
        }
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
