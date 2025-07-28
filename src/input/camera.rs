use bevy::prelude::*;

use crate::input::InputMode;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(InputMode::Camera), spawn_observers);
}

fn spawn_observers(mut commands: Commands) {
    commands.spawn((Observer::new(drag_camera), StateScoped(InputMode::Camera)));
    commands.spawn((Observer::new(zoom_camera), StateScoped(InputMode::Camera)));
}

fn drag_camera(
    trigger: Trigger<Pointer<Drag>>,
    mut camera_position: Single<&mut Transform, With<Camera>>,
) {
    let event = trigger.event();

    if event.button != PointerButton::Secondary {
        return;
    }

    camera_position.translation += event.delta.extend(0.0) * Vec3::new(-1.0, 1.0, 1.0) * 0.2;
}

fn zoom_camera(scroll: Trigger<Pointer<Scroll>>, proj: Single<&mut Projection, With<Camera>>) {
    if let Projection::Orthographic(ref mut ortho) = *proj.into_inner() {
        ortho.scale *= 1.0 - (scroll.y * 0.2);
    }
}
