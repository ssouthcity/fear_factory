use bevy::prelude::*;

const CAMERA_DRAG_SMOOTHING: f32 = 0.5;

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
    mut events: EventReader<Pointer<Drag>>,
    mut camera_position: Single<&mut Transform, With<Camera>>,
) {
    for event in events.read() {
        if event.button != PointerButton::Secondary {
            return;
        }

        camera_position.translation +=
            event.delta.extend(0.0) * Vec3::new(-1.0, 1.0, 1.0) * CAMERA_DRAG_SMOOTHING;
    }
}

fn zoom_camera(
    mut events: EventReader<Pointer<Scroll>>,
    projection: Single<&mut Projection, With<Camera>>,
) {
    let Projection::Orthographic(ref mut ortho) = *projection.into_inner() else {
        return;
    };

    for event in events.read() {
        ortho.scale *= 1.0 - (event.y * CAMERA_ZOOM_INTERVAL);
        ortho.scale = ortho.scale.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
    }
}
