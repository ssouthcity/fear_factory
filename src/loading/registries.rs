use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, time::common_conditions::repeating_after_delay};

use crate::{
    assets::tracking::is_finished_loading,
    gameplay::inventory::prelude::ItemDef,
    loading::{
        LoadingQueue, LoadingStages, loading_queue_is_empty,
        manifest::{Manifest, ManifestHandle},
        screen::SetLoadingText,
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingStages::LoadRegistries),
        (load_registries, set_flavor_text),
    );

    app.add_systems(
        Update,
        progress_to_next_loading_stage.run_if(
            in_state(LoadingStages::LoadRegistries)
                .and(loading_queue_is_empty)
                .and(is_finished_loading)
                .and(repeating_after_delay(Duration::from_secs(1))),
        ),
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Registry<T: Asset> {
    pub entries: HashMap<String, Handle<T>>,
}

impl<T: Asset> Default for Registry<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::default(),
        }
    }
}

fn load_registries(
    asset_server: Res<AssetServer>,
    manifest_handle: Res<ManifestHandle>,
    manifests: Res<Assets<Manifest>>,
    mut loading_queue: ResMut<LoadingQueue>,
) {
    let Some(manifest) = manifests.get(&manifest_handle.0) else {
        unreachable!("Manifest must be loaded before registry stage is reached");
    };

    let mut item_registry = Registry::<ItemDef>::default();
    for path in manifest.items.iter() {
        let id = path
            .split('/')
            .last()
            .unwrap()
            .trim_end_matches(".item.toml")
            .to_string();

        let item_handle: Handle<ItemDef> = asset_server.load(path);

        item_registry.entries.insert(id, item_handle.clone());

        loading_queue.0.push(item_handle.untyped());
    }
}

fn set_flavor_text(mut commands: Commands) {
    commands.trigger(SetLoadingText("Loading Registries".to_string()));
}

fn progress_to_next_loading_stage(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}
