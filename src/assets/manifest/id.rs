use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Reflect, Deserialize)]
#[serde(transparent)]
#[reflect(Component)]
pub struct Id<T> {
    pub value: String,
    #[serde(skip)]
    #[reflect(ignore)]
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(v: impl Into<String>) -> Self {
        Self {
            value: v.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id<{}>({:?})", std::any::type_name::<T>(), self.value)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}
