use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

use super::Manifest;

#[derive(Debug, Error)]
pub enum ManifestLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
}

pub struct ManifestLoader<T>(PhantomData<T>);

impl<T> Default for ManifestLoader<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> AssetLoader for ManifestLoader<T>
where
    T: for<'de> Deserialize<'de> + TypePath + Send + Sync,
{
    type Asset = Manifest<T>;
    type Settings = ();
    type Error = ManifestLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let mut manifest: Manifest<T> = toml::from_slice(&bytes)?;

        for (id, value) in manifest.entries.iter_mut() {
            value.id = id.clone();
        }

        Ok(manifest)
    }
}
