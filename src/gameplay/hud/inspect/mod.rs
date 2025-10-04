use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::gameplay::recipe::select::SelectedRecipe;

mod info;
mod select;

pub fn plugin(app: &mut App) {
    app.init_state::<InspectionMenuState>();
    app.init_resource::<InspectedEntity>();

    app.add_plugins((info::plugin, select::plugin));

    app.add_observer(on_inspect);

    app.add_systems(
        Update,
        close_menu.run_if(input_just_pressed(KeyCode::Escape)),
    );
}

#[derive(States, Reflect, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum InspectionMenuState {
    #[default]
    Closed,
    RecipeSelect,
    RecipeInspect,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct InspectedEntity(Entity);

impl Default for InspectedEntity {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

#[derive(EntityEvent, Reflect)]
pub struct Inspect {
    pub entity: Entity,
}

fn on_inspect(
    inspect: On<Inspect>,
    mut next_state: ResMut<NextState<InspectionMenuState>>,
    mut inspected_entity: ResMut<InspectedEntity>,
    selected_recipes: Query<&SelectedRecipe>,
) {
    let Ok(selected_recipe) = selected_recipes.get(inspect.entity) else {
        return;
    };

    inspected_entity.0 = inspect.entity;

    if selected_recipe.is_some() {
        next_state.set(InspectionMenuState::RecipeInspect);
    } else {
        next_state.set(InspectionMenuState::RecipeSelect);
    }
}

fn close_menu(mut next_state: ResMut<NextState<InspectionMenuState>>) {
    next_state.set(InspectionMenuState::Closed);
}
