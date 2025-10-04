use bevy::prelude::*;

const CAMERA_DRAG_SMOOTHING: f32 = 1.0;

const CAMERA_ZOOM_INTERVAL: f32 = 0.1;
const CAMERA_ZOOM_MIN: f32 = 0.25;
const CAMERA_ZOOM_MAX: f32 = 2.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera);
    app.add_systems(Update, (move_camera, zoom_camera));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}

fn move_camera(
    mut pointer_drags: MessageReader<Pointer<Drag>>,
    mut camera_position: Single<&mut Transform, With<Camera>>,
) {
    for pointer_drag in pointer_drags.read() {
        if pointer_drag.button != PointerButton::Secondary {
            return;
        }

        camera_position.translation +=
            pointer_drag.delta.extend(0.0) * Vec3::new(-1.0, 1.0, 1.0) * CAMERA_DRAG_SMOOTHING;
    }
}

fn zoom_camera(
    mut pointer_scrolls: MessageReader<Pointer<Scroll>>,
    projection: Single<&mut Projection, With<Camera>>,
) {
    let Projection::Orthographic(ref mut ortho) = *projection.into_inner() else {
        return;
    };

    for pointer_scroll in pointer_scrolls.read() {
        ortho.scale *= 1.0 - (pointer_scroll.y * CAMERA_ZOOM_INTERVAL);
        ortho.scale = ortho.scale.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
    }
}
