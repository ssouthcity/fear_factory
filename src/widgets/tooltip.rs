use bevy::prelude::*;

pub const TOOLTIP_POINTER_OFFSET: Vec2 = Vec2 { x: 0.0, y: -32.0 };

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_tooltip)
        .add_observer(on_show_tooltip)
        .add_observer(on_hide_tooltip)
        .add_observer(follow_mouse);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Tooltip;

#[derive(Event, Reflect, Debug)]
pub struct ShowTooltip(pub String);

#[derive(Event, Reflect, Debug)]
pub struct HideTooltip;

fn spawn_tooltip(mut commands: Commands) {
    commands.spawn((
        Name::new("Tooltip"),
        Tooltip,
        Node::default(),
        Text::default(),
        BackgroundColor(Color::hsl(210.0, 0.13, 0.5)),
        GlobalZIndex(100),
        Visibility::Hidden,
    ));
}

fn on_show_tooltip(
    show_tooltip: On<ShowTooltip>,
    tooltip_query: Single<(&mut Text, &mut Visibility), With<Tooltip>>,
) {
    let (mut text, mut visibility) = tooltip_query.into_inner();
    text.0 = show_tooltip.0.to_owned();
    *visibility = Visibility::Inherited;
}

fn on_hide_tooltip(
    _hide_tooltip: On<HideTooltip>,
    tooltip_query: Single<(&mut Text, &mut Visibility), With<Tooltip>>,
) {
    let (mut text, mut visibility) = tooltip_query.into_inner();
    text.0 = String::default();
    *visibility = Visibility::Hidden;
}

fn follow_mouse(
    pointer_move: On<Pointer<Move>>,
    tooltip_query: Single<&mut UiTransform, With<Tooltip>>,
) {
    let tooltip_origin = pointer_move.pointer_location.position + TOOLTIP_POINTER_OFFSET;

    let mut transform = tooltip_query.into_inner();
    transform.translation.x = px(tooltip_origin.x);
    transform.translation.y = px(tooltip_origin.y);
}
