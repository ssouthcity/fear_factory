use bevy::prelude::*;

use crate::{
    input::input_map::{Action, action_just_pressed},
    screens::Screen,
};

pub mod constants;
pub mod inspect;
pub mod inventory;
pub mod tabs;
pub mod tome;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((inventory::plugin, inspect::plugin, tabs::plugin));

    app.add_sub_state::<TomeMenu>();

    app.add_systems(
        Update,
        toggle_tome_open
            .run_if(in_state(Screen::Gameplay).and(action_just_pressed(Action::OpenTome))),
    );
}

#[derive(SubStates, Reflect, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(Screen = Screen::Gameplay)]
pub enum TomeMenu {
    #[default]
    None,
    Inventory,
    Inspect,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UITomeRoot;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UITomeTabs;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UITomeLeftPageRoot;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UITomeRightPageRoot;

fn toggle_tome_open(
    tome_menu: Res<State<TomeMenu>>,
    mut next_tome_menu: ResMut<NextState<TomeMenu>>,
) {
    next_tome_menu.set(match tome_menu.get() {
        TomeMenu::None => TomeMenu::Inventory,
        _ => TomeMenu::None,
    });
}

fn list_page() -> impl Bundle {
    (Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(8.0),
        width: percent(100.0),
        height: percent(100.0),
        padding: percent(5.0).all(),
        overflow: Overflow::scroll_y(),
        ..default()
    },)
}
