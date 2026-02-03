use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::repeating_after_delay};
use serde::Deserialize;

use crate::{
    assets::loaders::toml::{FromToml, TomlAssetPlugin},
    loading::{LoadingQueue, LoadingStages, loading_queue_is_empty, screen::SetLoadingText},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<Manifest>::extensions(&["manifest.toml"]));

    app.add_systems(
        OnEnter(LoadingStages::LoadManifest),
        (load_manifest, set_flavor_text),
    );

    app.add_systems(
        Update,
        progress_to_next_loading_stage.run_if(
            in_state(LoadingStages::LoadManifest)
                .and(loading_queue_is_empty)
                .and(repeating_after_delay(Duration::from_secs(1))),
        ),
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub(super) struct ManifestHandle(pub Handle<Manifest>);

#[derive(Deserialize)]
pub(super) struct RawManifest {
    items: Vec<String>,
}

#[derive(Asset, Reflect)]
pub(super) struct Manifest {
    pub items: Vec<String>,
}

impl FromToml for Manifest {
    type Raw = RawManifest;
    fn from_toml(raw: Self::Raw, _load_context: &mut bevy::asset::LoadContext) -> Self {
        Self { items: raw.items }
    }
}

fn load_manifest(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut loading_queue: ResMut<LoadingQueue>,
) {
    let handle = asset_server.load("fear_factory.manifest.toml");

    commands.insert_resource(ManifestHandle(handle.clone()));

    loading_queue.0.push(handle.untyped());
}

fn set_flavor_text(mut commands: Commands) {
    commands.trigger(SetLoadingText("Loading Manifest".to_string()));
}

fn progress_to_next_loading_stage(mut next_loading_stage: ResMut<NextState<LoadingStages>>) {
    next_loading_stage.set(LoadingStages::LoadRegistries);
}
