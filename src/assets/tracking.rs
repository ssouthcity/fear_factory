// Code "borrowed" from the semi-official 2d template:
// https://github.com/TheBevyFlock/bevy_new_2d/blob/main/src/asset_tracking.rs
use std::collections::VecDeque;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>();
    app.add_systems(PreUpdate, load_resource_assets);
}

pub trait LoadResource {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();
        let world = self.world_mut();
        let value = T::from_world(world);
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .waiting
            .push_back((handle.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));
        self
    }
}

type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
pub struct ResourceHandles {
    waiting: VecDeque<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
}

impl ResourceHandles {
    pub fn is_all_done(&self) -> bool {
        self.waiting.is_empty()
    }
}

fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, insert_fn));
                }
            }
        });
    });
}

pub fn is_finished_loading(handles: Res<ResourceHandles>) -> bool {
    handles.is_all_done()
}
