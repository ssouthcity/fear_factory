use bevy::{
    animation::{AnimatedBy, AnimationEvent, AnimationTargetId, animated_field, prelude::*},
    prelude::*,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen)
        .add_observer(on_splash_animation_finished);
}

#[derive(AnimationEvent, Clone)]
struct SplashFinished;

fn spawn_splash_screen(
    mut commands: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
) {
    let animation_target_name = Name::new("Studio");
    let animation_target_id = AnimationTargetId::from_name(&animation_target_name);

    let mut animation_clip = AnimationClip::default();

    animation_clip.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(TextFont::font_size),
            EasingCurve::new(0.0, 48.0, EaseFunction::Elastic(1.5)),
        ),
    );

    animation_clip.add_event(2.5, SplashFinished);

    let animation_clip_handle = animation_clips.add(animation_clip);
    let (animation_graph, animation_graph_node_index) =
        AnimationGraph::from_clip(animation_clip_handle);
    let animation_graph_handle = animation_graphs.add(animation_graph);

    let mut animation_player = AnimationPlayer::default();
    animation_player.play(animation_graph_node_index);

    let screen = commands
        .spawn((
            Name::new("Splash Screen"),
            DespawnOnExit(Screen::Splash),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            animation_player,
            AnimationGraphHandle(animation_graph_handle),
        ))
        .id();

    commands.spawn((
        Text::new("Studio"),
        TextFont::from_font_size(0.0),
        TextColor::WHITE,
        TextLayout::new_with_justify(Justify::Center),
        animation_target_name,
        animation_target_id,
        AnimatedBy(screen),
        ChildOf(screen),
    ));
}

fn on_splash_animation_finished(_: On<SplashFinished>, mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Loading);
}
