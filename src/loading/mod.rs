use bevy::prelude::*;

use crate::screens::Screen;

mod manifest;
mod registries;
mod screen;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<LoadingStages>();

    app.init_resource::<LoadingQueue>();

    app.add_plugins((manifest::plugin, registries::plugin, screen::plugin));

    app.add_systems(
        Update,
        clean_loading_queue.run_if(in_state(Screen::Loading)),
    );
}

/// Stages of loading
/// - Load the manifest file that collects other manifests
/// - Load entries of the manifest into separate registries
#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(Screen = Screen::Loading)]
enum LoadingStages {
    #[default]
    LoadManifest,
    LoadRegistries,
}

/// Contains a list of currently queued assets
/// Used to verify that loading has concluded
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct LoadingQueue(Vec<UntypedHandle>);

fn clean_loading_queue(asset_server: Res<AssetServer>, mut loading_queue: ResMut<LoadingQueue>) {
    loading_queue
        .0
        .retain(|handle| !asset_server.is_loaded(handle.id()));
}

pub fn loading_queue_is_empty(loading_queue: Res<LoadingQueue>) -> bool {
    loading_queue.0.len() == 0
}
