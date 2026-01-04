use bevy::prelude::*;

use crate::gameplay::tome::{TomeMenu, tome_plugin::TomePlugin};

pub mod porter_management;
pub mod recipe_select;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TomePlugin::<InspectTabs> {
        menu: TomeMenu::Inspect,
        tabs: vec![
            ("Recipe", InspectTabs::RecipeSelect),
            ("Porters", InspectTabs::PorterManagement),
        ],
    });

    app.add_plugins((porter_management::plugin, recipe_select::plugin));

    app.add_observer(on_inspect);
}

#[derive(EntityEvent, Reflect)]
pub struct Inspect {
    pub entity: Entity,
}

#[derive(SubStates, Component, Reflect, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(TomeMenu = TomeMenu::Inspect)]
#[reflect(Component)]
pub enum InspectTabs {
    #[default]
    RecipeSelect,
    PorterManagement,
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct Inspected(pub Entity);

fn on_inspect(
    inspect: On<Inspect>,
    mut commands: Commands,
    mut next_tome_menu: ResMut<NextState<TomeMenu>>,
) {
    commands.insert_resource(Inspected(inspect.entity));
    next_tome_menu.set(TomeMenu::Inspect);
}
