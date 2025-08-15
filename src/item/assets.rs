use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{assets::manifest::Id, item::Item};

pub fn plugin(app: &mut App) {
    app.register_type::<ItemAssets>();

    app.init_resource::<ItemAssets>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ItemAssets {
    pub aseprite: Handle<Aseprite>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let aseprite = asset_server.load("items.aseprite");
        Self { aseprite }
    }
}

impl ItemAssets {
    fn slice(&self, item: Id<Item>) -> impl Bundle {
        AseSlice {
            aseprite: self.aseprite.clone(),
            name: item.id,
        }
    }

    pub fn sprite(&self, item: Id<Item>) -> impl Bundle {
        (Sprite::sized(Vec2::splat(16.0)), self.slice(item))
    }
}
