use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::item::ItemID;

pub fn plugin(app: &mut App) {
    app.register_type::<ItemAssets>();

    app.init_resource::<ItemAssets>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ItemAssets {
    aseprite: Handle<Aseprite>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let aseprite = asset_server.load("items.aseprite");
        Self { aseprite }
    }
}

impl ItemAssets {
    fn slice(&self, item: ItemID) -> impl Bundle {
        AseSlice {
            aseprite: self.aseprite.clone(),
            name: item.0.to_string(),
        }
    }

    pub fn sprite(&self, item: ItemID) -> impl Bundle {
        (Sprite::sized(Vec2::splat(16.0)), self.slice(item))
    }

    pub fn ui_icon(&self, item: ItemID) -> impl Bundle {
        (ImageNode::default(), self.slice(item))
    }
}
