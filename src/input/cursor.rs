use bevy::prelude::*;

use crate::gameplay::FactorySystems;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CursorPosition>();
    app.init_resource::<CursorPosition>();

    app.add_systems(
        Update,
        record_cursor_position
            .in_set(FactorySystems::Input)
            .run_if(on_event::<CursorMoved>),
    );
}

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct CursorPosition(pub Vec2);

fn record_cursor_position(
    mut events: EventReader<CursorMoved>,
    camera_query: Single<(&GlobalTransform, &Camera)>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera_position, camera) = *camera_query;

    for event in events.read() {
        if let Ok(pos) = camera.viewport_to_world_2d(camera_position, event.position) {
            *cursor_position = CursorPosition(pos);
        }
    }
}
