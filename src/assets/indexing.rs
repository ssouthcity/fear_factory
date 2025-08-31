use std::{collections::HashMap, marker::PhantomData};

use bevy::prelude::*;

pub trait Indexable {
    fn index(&self) -> &String;
}

pub struct AssetIndexPlugin<T>(PhantomData<T>);

impl<T> Default for AssetIndexPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Plugin for AssetIndexPlugin<T>
where
    T: Asset + Indexable,
{
    fn build(&self, app: &mut App) {
        app.register_type::<IndexMap<T>>();
        app.init_resource::<IndexMap<T>>();

        app.add_systems(Update, populate_index::<T>);
    }
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct IndexMap<T: Asset>(HashMap<String, AssetId<T>>);

impl<T> Default for IndexMap<T>
where
    T: Asset,
{
    fn default() -> Self {
        Self(HashMap::default())
    }
}

fn populate_index<T: Asset + Indexable>(
    mut events: EventReader<AssetEvent<T>>,
    assets: Res<Assets<T>>,
    mut index_map: ResMut<IndexMap<T>>,
) {
    for event in events.read() {
        if let AssetEvent::Added { id } = event {
            let Some(asset) = assets.get(*id) else {
                continue;
            };

            index_map.insert(asset.index().to_owned(), *id);
        }
    }
}
