use bevy::prelude::*;

mod build;
mod camera;
mod power_line;
mod view;

pub fn plugin(app: &mut App) {
    app.init_state::<InputMode>();

    app.add_plugins((
        camera::plugin,
        view::plugin,
        build::plugin,
        power_line::plugin,
    ));

    app.init_resource::<CursorPosition>()
        .add_systems(Update, track_cursor_position);
}

#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Default, Reflect)]
#[states(scoped_entities)]
pub enum InputMode {
    #[default]
    Camera,
    View,
    Build,
    PowerLine,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct CursorPosition(Vec2);

fn track_cursor_position(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.0.viewport_to_world_2d(camera.1, cursor).ok())
    {
        cursor_position.0 = world_position;
    }
}
