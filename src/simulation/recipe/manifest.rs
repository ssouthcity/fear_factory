use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    assets::manifest::Id,
    simulation::{item::Item, machine::StructureTemplate},
};

#[derive(Debug, Deserialize, TypePath)]
pub struct Recipe {
    pub name: String,
    #[serde(default)]
    pub input: HashMap<Id<Item>, u32>,
    #[serde(default)]
    pub output: HashMap<Id<Item>, u32>,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    #[serde(default)]
    pub tags: Vec<RecipeTags>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum RecipeTags {
    StructureId(Id<StructureTemplate>),
}
