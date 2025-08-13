use std::{collections::HashMap, marker::PhantomData};

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    ecs::system::SystemParam,
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize, Resource)]
pub struct Manifest<T: TypePath + Sync + Send> {
    pub items: HashMap<String, T>,
}

#[derive(Debug, Error)]
pub enum RawManifestLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
}

pub struct ManifestLoader<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for ManifestLoader<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> AssetLoader for ManifestLoader<T>
where
    T: for<'de> Deserialize<'de> + TypePath + Sync + Send + 'static,
{
    type Asset = Manifest<T>;
    type Settings = ();
    type Error = RawManifestLoaderError;

    fn extensions(&self) -> &[&str] {
        &["manifest.toml"]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let manifest: Manifest<T> = toml::from_slice(&bytes)?;
        Ok(manifest)
    }
}

pub struct ManifestPlugin<T> {
    pub path: &'static str,
    _phantom: PhantomData<T>,
}

impl<T> ManifestPlugin<T> {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
            _phantom: PhantomData,
        }
    }
}

impl<T> Plugin for ManifestPlugin<T>
where
    T: for<'de> Deserialize<'de> + TypePath + Sync + Send + 'static,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<Manifest<T>>()
            .register_asset_loader(ManifestLoader::<T>::default())
            .insert_resource(ManifestPath::<T>::new(self.path))
            .insert_resource(ManifestHandle::<T>::default())
            .add_systems(Startup, load_manifest::<T>);
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct ManifestPath<T> {
    path: &'static str,
    _phantom: PhantomData<T>,
}

impl<T> ManifestPath<T> {
    fn new(path: &'static str) -> Self {
        Self {
            path,
            _phantom: PhantomData,
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct ManifestHandle<T: TypePath + Sync + Send>(Handle<Manifest<T>>);

impl<T: TypePath + Sync + Send> Default for ManifestHandle<T> {
    fn default() -> Self {
        Self(Handle::<Manifest<T>>::default())
    }
}

fn load_manifest<T: TypePath + Sync + Send + 'static>(
    path: Res<ManifestPath<T>>,
    asset_server: Res<AssetServer>,
    mut handle: ResMut<ManifestHandle<T>>,
) {
    handle.0 = asset_server.load(path.path);
}

#[derive(SystemParam)]
pub struct ManifestParam<'w, T: TypePath + Sync + Send> {
    assets: Res<'w, Assets<Manifest<T>>>,
    handle: Res<'w, ManifestHandle<T>>,
}

impl<'w, T: TypePath + Sync + Send> ManifestParam<'w, T> {
    pub fn get(&self) -> Option<&Manifest<T>> {
        self.assets.get(&self.handle.0)
    }
}
