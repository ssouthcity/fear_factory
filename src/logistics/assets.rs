use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::assets::LoadResource;

pub fn plugin(app: &mut App) {
    app.register_type::<LogisticAssets>()
        .load_resource::<LogisticAssets>();
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct LogisticAssets {
    pub conveyor_belt: Handle<Aseprite>,
    pub conveyor_holes: Handle<Aseprite>,
}

impl FromWorld for LogisticAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            conveyor_belt: asset_server.load("conveyor.aseprite"),
            conveyor_holes: asset_server.load("conveyor_holes.aseprite"),
        }
    }
}
