use bevy::prelude::*;

use crate::gameplay::hud::tome::{TomeTab, UITomeLeftPageRoot, widgets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::People), spawn_people_grid);
}

fn spawn_people_grid(mut commands: Commands, left_page: Single<Entity, With<UITomeLeftPageRoot>>) {
    commands.spawn((
        widgets::list_page(),
        ChildOf(*left_page),
        DespawnOnExit(TomeTab::People),
        children![Text::new("People")],
    ));
}
