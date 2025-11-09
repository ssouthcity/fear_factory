use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CursorPosition>();

    app.add_observer(record_cursor_position);
}

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct CursorPosition(pub Vec2);

fn record_cursor_position(
    pointer_move: On<Pointer<Move>>,
    camera_query: Single<(&GlobalTransform, &Camera)>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera_position, camera) = *camera_query;

    if let Ok(pos) =
        camera.viewport_to_world_2d(camera_position, pointer_move.pointer_location.position)
    {
        *cursor_position = CursorPosition(pos);
    }
}
