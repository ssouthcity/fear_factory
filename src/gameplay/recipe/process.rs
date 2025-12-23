use bevy::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        FactorySystems,
        item::stack::Stack,
        recipe::{Input, Output, assets::RecipeDef, select::SelectedRecipe},
        storage::OutputStorage,
    },
};

use super::progress::on_progress_state_add;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (consume_input, progress_work, produce_output)
            .chain()
            .in_set(FactorySystems::Work),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_progress_state_add)]
pub enum ProcessState {
    #[default]
    InsufficientInput,
    Working(Timer),
    Completed,
}

impl ProcessState {
    pub fn progress(&self) -> f32 {
        match self {
            Self::InsufficientInput => 0.0,
            Self::Working(timer) => timer.fraction(),
            Self::Completed => 1.0,
        }
    }
}

fn consume_input(
    query: Query<(&mut ProcessState, &Storage, &SelectedRecipe)>,
    mut input_query: Query<(&mut Stack, &Input)>,
    recipes: Res<Assets<RecipeDef>>,
    recipe_index: Res<IndexMap<RecipeDef>>,
) {
    for (mut state, storage, selected_recipe) in query {
        if !matches!(*state, ProcessState::InsufficientInput) {
            continue;
        }

        if !storage
            .iter()
            .filter_map(|stored| input_query.get(stored).ok())
            .all(|(stack, input)| stack.quantity >= input.quantity)
        {
            continue;
        }

        for stored in storage.iter() {
            if let Ok((mut stack, input)) = input_query.get_mut(stored) {
                stack.quantity = stack.quantity.saturating_sub(input.quantity);
            }
        }

        let recipe = recipe_index
            .get(selected_recipe.0.as_str())
            .and_then(|asset_id| recipes.get(*asset_id))
            .expect("Selected recipe refers to non-existent recipe");

        let timer = Timer::new(recipe.duration, TimerMode::Once);

        *state = ProcessState::Working(timer);
    }
}

fn progress_work(query: Query<&mut ProcessState>, time: Res<Time>) {
    for mut state in query {
        let ProcessState::Working(ref mut timer) = *state else {
            continue;
        };

        if !timer.tick(time.delta()).is_finished() {
            continue;
        }

        *state = ProcessState::Completed;
    }
}

fn produce_output(
    query: Query<(&mut ProcessState, &OutputStorage)>,
    mut output_query: Query<(&mut Stack, &Output)>,
) {
    for (mut state, storage) in query {
        if !matches!(*state, ProcessState::Completed) {
            continue;
        }

        for stored in storage.iter() {
            let Ok((mut stack, output)) = output_query.get_mut(stored) else {
                continue;
            };

            stack.quantity = stack.quantity.saturating_add(output.quantity);
        }

        *state = ProcessState::InsufficientInput;
    }
}
