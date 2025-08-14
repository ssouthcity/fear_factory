use bevy::{platform::collections::HashSet, prelude::*};

pub fn plugin(app: &mut App) {
    app.register_type::<ItemID>();
    app.register_type::<ItemLexicon>();

    app.init_resource::<ItemLexicon>();

    app.add_systems(Startup, register_items);
}

#[derive(Component, Hash, PartialEq, Eq, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(immutable)]
pub struct ItemID(pub String);

impl std::fmt::Display for ItemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemID({})", self.0)
    }
}

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct ItemLexicon(pub HashSet<ItemID>);

fn register_items(mut lexicon: ResMut<ItemLexicon>) {
    lexicon.insert(ItemID("coal".into()));
    lexicon.insert(ItemID("iron_ore".into()));
    lexicon.insert(ItemID("iron_ingot".into()));
}
