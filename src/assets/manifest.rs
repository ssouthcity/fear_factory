use std::{collections::HashMap, marker::PhantomData};

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize)]
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
    _phantom: PhantomData<T>,
}

impl<T> Default for ManifestPlugin<T> {
    fn default() -> Self {
        Self {
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
            .register_asset_loader(ManifestLoader::<T>::default());
    }
}
