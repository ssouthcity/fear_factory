use std::marker::PhantomData;

use bevy::prelude::*;
use serde::Deserialize;

use super::{Manifest, ManifestLoader};

pub struct ManifestPlugin<T>(PhantomData<T>);

impl<T> Default for ManifestPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Plugin for ManifestPlugin<T>
where
    T: for<'de> Deserialize<'de> + TypePath + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<Manifest<T>>()
            .init_asset_loader::<ManifestLoader<T>>();
    }
}
