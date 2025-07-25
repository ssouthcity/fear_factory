use bevy::prelude::*;

use crate::power::FuseBroke;

pub fn plugin(app: &mut App) {
    app.add_observer(on_fuse_broke);
}

fn on_fuse_broke(
    _trigger: Trigger<FuseBroke>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let audio_handle = asset_server.load("blown-fuse.ogg");
    commands.spawn((AudioPlayer::new(audio_handle), PlaybackSettings::DESPAWN));
}
