use bevy::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        FactorySystems,
        item::stack::{Full, Stack},
        recipe::{InputOf, Inputs, OutputOf, Outputs, assets::RecipeDef, select::SelectedRecipe},
    },
};

use super::progress::on_progress_state_add;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
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
    query: Query<(&mut ProcessState, &Inputs, &SelectedRecipe)>,
    mut input_entities: Query<(&mut Stack, &InputOf)>,
    recipes: Res<Assets<RecipeDef>>,
    recipe_index: Res<IndexMap<RecipeDef>>,
) {
    for (mut state, inputs, selected_recipe) in query {
        if !matches!(*state, ProcessState::InsufficientInput) {
            continue;
        }

        if !inputs.iter().all(|input| {
            input_entities
                .get(input)
                .is_ok_and(|(stack, input_of)| stack.quantity >= input_of.required_quantity)
        }) {
            continue;
        }

        for input in inputs.iter() {
            if let Ok((mut stack, input_of)) = input_entities.get_mut(input) {
                stack.quantity -= input_of.required_quantity;
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
    query: Query<(&mut ProcessState, &Outputs)>,
    mut output_entities: Query<(&mut Stack, &OutputOf), Without<Full>>,
) {
    for (mut state, outputs) in query {
        if !matches!(*state, ProcessState::Completed) {
            continue;
        }

        if !outputs
            .iter()
            .any(|output| output_entities.get(output).is_ok())
        {
            continue;
        }

        for output in outputs.iter() {
            let Ok((mut stack, output_of)) = output_entities.get_mut(output) else {
                continue;
            };

            stack.quantity += output_of.output_quantity;
        }

        *state = ProcessState::InsufficientInput;
    }
}
