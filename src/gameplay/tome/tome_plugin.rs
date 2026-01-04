use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::gameplay::tome::{
    TomeMenu, UITomeLeftPageRoot, UITomeRightPageRoot, UITomeRoot, UITomeTabs, constants, tabs,
};

pub struct TomePlugin<Tabs: States + Component> {
    pub menu: TomeMenu,
    pub tabs: Vec<(&'static str, Tabs)>,
}

impl<Tabs: SubStates + Component> Plugin for TomePlugin<Tabs> {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<Tabs>();

        app.insert_resource(TomeConfig {
            menu: self.menu,
            tabs: self.tabs.clone(),
        });

        app.add_systems(OnEnter(self.menu), spawn_tome::<Tabs>);

        app.add_systems(
            Update,
            tabs::sync_tab_checked::<Tabs>.run_if(state_exists::<Tabs>),
        );
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct TomeConfig<Tabs: FreelyMutableState + Component> {
    menu: TomeMenu,
    tabs: Vec<(&'static str, Tabs)>,
}

fn spawn_tome<Tabs: FreelyMutableState + Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tome_config: Res<TomeConfig<Tabs>>,
) {
    let container = commands
        .spawn((
            Name::new("Tome Container"),
            UITomeRoot,
            DespawnOnExit(tome_config.menu),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();

    let tome = commands
        .spawn((
            Name::new("Tome"),
            ChildOf(container),
            Node {
                position_type: PositionType::Relative,
                ..default()
            },
        ))
        .id();

    let tabs = commands
        .spawn((
            Name::new("Tabs"),
            UITomeTabs,
            ChildOf(tome),
            Node {
                position_type: PositionType::Absolute,
                top: px(64.0),
                right: px(constants::PAGE_WIDTH * 2.0 - 32.0),
                flex_direction: FlexDirection::Column,
                row_gap: px(8.0),
                ..default()
            },
            ZIndex(10),
        ))
        .id();

    for (name, value) in tome_config.tabs.iter() {
        commands.spawn((ChildOf(tabs), tabs::tab(&asset_server, name, value.clone())));
    }

    let _left_page = commands
        .spawn((
            Name::new("Left Page"),
            UITomeLeftPageRoot,
            ChildOf(tome),
            Node {
                width: px(constants::PAGE_WIDTH),
                height: px(constants::PAGE_HEIGHT),
                padding: percent(6.0).all(),
                ..default()
            },
            ImageNode {
                image: asset_server.load("sprites/hud/tome_left.png"),
                ..default()
            },
        ))
        .id();

    let _right_page = commands
        .spawn((
            Name::new("Right Page"),
            UITomeRightPageRoot,
            ChildOf(tome),
            Node {
                width: px(constants::PAGE_WIDTH),
                height: px(constants::PAGE_HEIGHT),
                padding: percent(6.0).all(),
                ..default()
            },
            ImageNode {
                image: asset_server.load("sprites/hud/tome_right.png"),
                ..default()
            },
        ))
        .id();
}
