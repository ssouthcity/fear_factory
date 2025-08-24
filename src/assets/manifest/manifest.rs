use std::{collections::HashMap, ops::Deref};

use bevy::prelude::*;
use serde::Deserialize;

use super::Id;

#[derive(Debug, Clone, Deserialize)]
pub struct Definition<T> {
    #[serde(skip, default = "unset_id")]
    pub id: Id<T>,
    #[serde(flatten)]
    pub value: T,
}

fn unset_id<T>() -> Id<T> {
    Id::new("<unset>")
}

impl<T> Deref for Definition<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Manifest<T: TypePath + Sync + Send> {
    pub(super) entries: HashMap<Id<T>, Definition<T>>,
}

impl<T> Manifest<T>
where
    T: TypePath + Sync + Send,
{
    pub fn get(&self, id: &Id<T>) -> Option<&Definition<T>> {
        self.entries.get(id)
    }

    pub fn contains(&self, id: &Id<T>) -> bool {
        self.entries.contains_key(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Id<T>, &Definition<T>)> {
        self.entries.iter()
    }
}
