use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};

use super::process::ProcessState;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_progress_bars);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = ProgressBarFillOf, linked_spawn)]
pub struct ProgressBarFill(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ProgressBarFill)]
pub struct ProgressBarFillOf(Entity);

pub fn on_progress_state_add(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    world.commands().spawn((
        Name::new("Progress Bar"),
        ChildOf(entity),
        Transform::from_xyz(0.0, 48.0, 100.0),
        Sprite::from_color(Color::BLACK, Vec2::new(64.0, 16.0)),
        children![(
            Name::new("Progress Bar Fill"),
            ProgressBarFillOf(entity),
            Transform::from_xyz(-32.0, 0.0, 1.0),
            Sprite {
                color: Color::linear_rgb(0.0, 0.8, 0.1),
                rect: Some(Rect::new(0.0, 0.0, 64.0, 16.0)),
                ..default()
            },
            Anchor::CENTER_LEFT,
        )],
    ));
}

fn update_progress_bars(
    work_states: Query<&ProcessState>,
    progress_bars: Query<(&mut Sprite, &ProgressBarFillOf)>,
) {
    for (mut sprite, progress_bar_of) in progress_bars {
        let Ok(state) = work_states.get(progress_bar_of.0) else {
            continue;
        };

        let Some(rect) = sprite.rect else {
            continue;
        };

        sprite.custom_size = Some(Vec2::new(rect.width() * state.progress(), rect.height()));
    }
}
