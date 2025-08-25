use bevy::prelude::*;

use crate::{assets::LoadResource, simulation::power::FuseBlown};

pub fn plugin(app: &mut App) {
    app.register_type::<AudioAssets>()
        .load_resource::<AudioAssets>();

    app.add_observer(on_fuse_broke);
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct AudioAssets {
    blown_fuse: Handle<AudioSource>,
}

impl FromWorld for AudioAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            blown_fuse: assets.load("blown-fuse.ogg"),
        }
    }
}

fn on_fuse_broke(
    _trigger: Trigger<FuseBlown>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    commands.spawn((
        AudioPlayer::new(audio_assets.blown_fuse.clone()),
        PlaybackSettings::DESPAWN,
    ));
}
