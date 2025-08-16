use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::assets::LoadResource;

pub fn plugin(app: &mut App) {
    app.register_type::<WorldAssets>()
        .load_resource::<WorldAssets>();
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct WorldAssets {
    pub aseprite: Handle<Aseprite>,
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            aseprite: assets.load("terrain.aseprite"),
        }
    }
}
