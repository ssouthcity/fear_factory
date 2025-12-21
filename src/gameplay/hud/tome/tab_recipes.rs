use bevy::prelude::*;

use crate::gameplay::hud::tome::{TomeTab, UITomeLeftPageRoot, widgets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::Recipes), spawn_recipe_list);
}

fn spawn_recipe_list(mut commands: Commands, left_page: Single<Entity, With<UITomeLeftPageRoot>>) {
    commands.spawn((
        widgets::list_page(),
        ChildOf(*left_page),
        DespawnOnExit(TomeTab::Recipes),
        children![Text::new("Recipes")],
    ));
}
