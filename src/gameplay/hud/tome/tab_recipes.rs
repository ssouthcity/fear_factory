use bevy::prelude::*;

use crate::{
    gameplay::{
        hud::tome::{TomeTab, UITomeLeftPageRoot},
        recipe::assets::RecipeDef,
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::Recipes), spawn_recipe_list);
}

fn spawn_recipe_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    recipe_defs: Res<Assets<RecipeDef>>,
) {
    let recipe_list = commands
        .spawn((
            super::widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::Recipes),
        ))
        .id();

    for (asset_id, _) in recipe_defs.iter() {
        commands.spawn((widgets::recipe_plate(asset_id), ChildOf(recipe_list)));
    }
}
