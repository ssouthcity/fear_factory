use bevy::prelude::*;

const CAMERA_DRAG_SCALING: f32 = 1.0;

const CAMERA_ZOOM_SCALING: f32 = 0.1;
const CAMERA_ZOOM_MIN: f32 = 0.25;
const CAMERA_ZOOM_MAX: f32 = 4.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera);
    app.add_observer(move_camera);
    app.add_observer(zoom_camera);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}

fn move_camera(
    pointer_drag: On<Pointer<Drag>>,
    mut camera: Single<(&mut Transform, &Projection), With<Camera>>,
) {
    if pointer_drag.button != PointerButton::Secondary {
        return;
    }

    let Projection::Orthographic(ortho) = camera.1 else {
        return;
    };

    camera.0.translation += pointer_drag.delta.extend(0.0)
        * ortho.scale
        * Vec3::new(-1.0, 1.0, 1.0)
        * CAMERA_DRAG_SCALING;
}

fn zoom_camera(
    pointer_scroll: On<Pointer<Scroll>>,
    mut projection: Single<&mut Projection, With<Camera>>,
) {
    let Projection::Orthographic(ref mut ortho) = **projection else {
        return;
    };

    ortho.scale *= 1.0 - (pointer_scroll.y * CAMERA_ZOOM_SCALING);
    ortho.scale = ortho.scale.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
}
