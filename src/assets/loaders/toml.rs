use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoader, LoadContext},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TomlLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
}

pub trait FromToml {
    type Raw: for<'de> Deserialize<'de>;

    fn from_toml(raw: Self::Raw, load_context: &mut LoadContext) -> Self;
}

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
    T: Asset + FromToml,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>();

        app.register_asset_loader(TomlAssetLoader::<T> {
            extensions: self.extensions.to_owned(),
            _marker: PhantomData,
        });
    }
}

#[derive(TypePath)]
pub struct TomlAssetLoader<T> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<T>,
}

impl<T> AssetLoader for TomlAssetLoader<T>
where
    T: Asset + FromToml,
{
    type Asset = T;
    type Settings = ();
    type Error = TomlLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let raw: T::Raw = toml::from_slice(&bytes)?;
        Ok(T::from_toml(raw, load_context))
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
