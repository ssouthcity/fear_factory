use bevy::prelude::*;

use crate::FactorySystems;

pub fn plugin(app: &mut App) {
    app.register_type::<PowerSockets>();
    app.register_type::<CurrentDraggedSocket>();

    app.init_resource::<CurrentDraggedSocket>();

    app.register_type::<PowerSocketsLinked>();
    app.add_event::<PowerSocketsLinked>();

    app.add_observer(on_power_socket_drag_start)
        .add_observer(on_power_socket_drag)
        .add_observer(on_power_socket_drag_end)
        .add_observer(on_power_socket_drag_drop);

    app.add_systems(
        Update,
        (
            update_connections.in_set(FactorySystems::Build),
            draw_current_dragged_socket.in_set(FactorySystems::UI),
        ),
    );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct CurrentDraggedSocket {
    socket: Option<Entity>,
    drag_delta: Vec2,
}

#[derive(Event, Reflect)]
pub struct PowerSocketsLinked(pub Entity, pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(PowerSocketConnections, Pickable)]
pub struct PowerSockets(u8);

impl PowerSockets {
    pub fn single() -> PowerSockets {
        PowerSockets(1)
    }

    pub fn multiple(amount: u8) -> PowerSockets {
        PowerSockets(amount)
    }
}

impl Default for PowerSockets {
    fn default() -> PowerSockets {
        PowerSockets::single()
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PowerSocketConnections(u8);

fn on_power_socket_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    sockets: Query<(&PowerSockets, &PowerSocketConnections)>,
    mut current_dragged_socket: ResMut<CurrentDraggedSocket>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let Ok((PowerSockets(available_connections), PowerSocketConnections(active_connections))) =
        sockets.get(trigger.target())
    else {
        return;
    };

    if active_connections >= available_connections {
        return;
    }

    current_dragged_socket.socket = Some(trigger.target());

    trigger.propagate(false);
}

fn on_power_socket_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    sockets: Query<&PowerSockets>,
    mut current_dragged_socket: ResMut<CurrentDraggedSocket>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    if !sockets.contains(trigger.target()) {
        return;
    }

    current_dragged_socket.drag_delta = trigger.event().distance;

    trigger.propagate(false);
}

fn on_power_socket_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    sockets: Query<&PowerSockets>,
    mut current_dragged_socket: ResMut<CurrentDraggedSocket>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    if !sockets.contains(trigger.target()) {
        return;
    }

    current_dragged_socket.socket = None;

    trigger.propagate(false);
}

fn on_power_socket_drag_drop(
    mut trigger: Trigger<Pointer<DragDrop>>,
    sockets: Query<(&PowerSockets, &PowerSocketConnections)>,
    mut events: EventWriter<PowerSocketsLinked>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let event = trigger.event();

    let Ok((
        PowerSockets(dropped_available_connections),
        PowerSocketConnections(dropped_active_connections),
    )) = sockets.get(event.dropped)
    else {
        return;
    };

    if dropped_active_connections >= dropped_available_connections {
        return;
    }

    let Ok((
        PowerSockets(target_available_connections),
        PowerSocketConnections(target_active_connections),
    )) = sockets.get(event.target)
    else {
        return;
    };

    if target_active_connections >= target_available_connections {
        return;
    }

    events.write(PowerSocketsLinked(event.target, event.dropped));

    trigger.propagate(false);
}

fn update_connections(
    mut events: EventReader<PowerSocketsLinked>,
    mut power_socket_connections: Query<&mut PowerSocketConnections>,
) {
    for event in events.read() {
        if let Ok(mut connections) = power_socket_connections.get_mut(event.0) {
            connections.0 += 1;
        }

        if let Ok(mut connections) = power_socket_connections.get_mut(event.1) {
            connections.0 += 1;
        }
    }
}

fn draw_current_dragged_socket(
    mut gizmos: Gizmos,
    current_dragged_socket: Res<CurrentDraggedSocket>,
    transforms: Query<&Transform>,
) {
    let Some(socket) = current_dragged_socket.socket else {
        return;
    };

    let start = transforms.get(socket).unwrap().translation.truncate();

    gizmos.line_2d(
        start,
        start + current_dragged_socket.drag_delta * Vec2::new(1.0, -1.0),
        Color::BLACK.with_alpha(0.5),
    );
}
