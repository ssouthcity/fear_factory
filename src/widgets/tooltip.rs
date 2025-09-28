use bevy::prelude::*;

pub const TOOLTIP_POINTER_OFFSET: Vec2 = Vec2 { x: 0.0, y: -32.0 };

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Tooltip>();

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
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        Text::new("hello world!"),
        BackgroundColor(Color::hsl(210.0, 0.13, 0.5)),
        GlobalZIndex(100),
        Visibility::Hidden,
    ));
}

fn on_show_tooltip(
    trigger: Trigger<ShowTooltip>,
    tooltip_query: Query<(&mut Text, &mut Visibility), With<Tooltip>>,
) {
    for (mut text, mut visibility) in tooltip_query {
        text.0 = trigger.0.to_owned();
        *visibility = Visibility::Inherited;
    }
}

fn on_hide_tooltip(
    _trigger: Trigger<HideTooltip>,
    tooltip_query: Query<(&mut Text, &mut Visibility), With<Tooltip>>,
) {
    for (mut text, mut visibility) in tooltip_query {
        text.0 = String::default();
        *visibility = Visibility::Hidden;
    }
}

fn follow_mouse(trigger: Trigger<Pointer<Move>>, tooltip_query: Query<&mut Node, With<Tooltip>>) {
    for mut node in tooltip_query {
        let tooltip_origin = trigger.pointer_location.position + TOOLTIP_POINTER_OFFSET;

        node.left = Val::Px(tooltip_origin.x);
        node.top = Val::Px(tooltip_origin.y);
    }
}
