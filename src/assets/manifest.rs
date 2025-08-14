use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData, ops::Deref};

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    ecs::system::SystemParam,
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Reflect, Component)]
pub struct Id<T> {
    pub id: String,
    #[reflect(ignore)]
    _phantom: PhantomData<T>,
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id<{}>({:?})", std::any::type_name::<T>(), self.id)
    }
}

impl<T> From<&'static str> for Id<T> {
    fn from(value: &'static str) -> Self {
        Self {
            id: value.into(),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<String> for Id<T> {
    fn from(value: String) -> Self {
        Self {
            id: value,
            _phantom: PhantomData,
        }
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id: String = Deserialize::deserialize(deserializer)?;
        Ok(Id {
            id,
            _phantom: PhantomData,
        })
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T> Eq for Id<T> {}

#[derive(Asset, TypePath, Deserialize, Deref)]
pub struct Manifest<T: TypePath + Sync + Send>(HashMap<Id<T>, T>);

pub struct Definition<'a, T> {
    pub id: Id<T>,
    pub definition: &'a T,
}

impl<'a, T> Deref for Definition<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.definition
    }
}

impl<T> Manifest<T>
where
    T: TypePath + Sync + Send,
{
    pub fn get(&'_ self, id: &Id<T>) -> Option<Definition<'_, T>> {
        self.0.get(id).map(|definition| Definition {
            id: id.clone(),
            definition,
        })
    }
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
    pub fn read(&self) -> Option<&Manifest<T>> {
        self.assets.get(&self.handle.0)
    }
}
