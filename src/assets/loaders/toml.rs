use std::marker::PhantomData;

use bevy::{asset::AssetLoader, prelude::*};
use serde::Deserialize;
use thiserror::Error;

pub struct TomlAssetPlugin<T> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<T>,
}

impl<T> TomlAssetPlugin<T> {
    pub fn extensions(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            _marker: PhantomData,
        }
    }
}

impl<T> Plugin for TomlAssetPlugin<T>
where
    T: Asset + for<'de> Deserialize<'de>,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>();

        app.register_asset_loader(TomlAssetLoader::<T> {
            extensions: self.extensions.to_owned(),
            _marker: PhantomData,
        });
    }
}

pub struct TomlAssetLoader<T> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum TomlLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
}

impl<T> AssetLoader for TomlAssetLoader<T>
where
    T: Asset + for<'de> Deserialize<'de>,
{
    type Asset = T;
    type Settings = ();
    type Error = TomlLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset: T = toml::from_slice(&bytes)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
