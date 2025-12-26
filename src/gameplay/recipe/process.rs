use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    item::inventory::Inventory,
    recipe::{assets::Recipe, select::SelectedRecipe},
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
    query: Query<(&mut ProcessState, &mut Inventory, &SelectedRecipe)>,
    recipes: Res<Assets<Recipe>>,
) {
    for (mut state, mut inventory, selected_recipe) in query {
        if !matches!(*state, ProcessState::InsufficientInput) {
            continue;
        }

        let Some(recipe) = recipes.get(&selected_recipe.0) else {
            continue;
        };

        if !recipe.input.iter().all(|(item_id, required_amount)| {
            inventory
                .items
                .get(item_id)
                .is_some_and(|quantity| quantity >= required_amount)
        }) {
            continue;
        }

        for (item_id, required_amount) in recipe.input.iter() {
            inventory
                .items
                .entry(*item_id)
                .and_modify(|quantity| *quantity -= required_amount);
        }

        let Some(recipe) = recipes.get(&selected_recipe.0) else {
            continue;
        };

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
    query: Query<(&mut ProcessState, &mut Inventory, &SelectedRecipe)>,
    recipes: Res<Assets<Recipe>>,
) {
    for (mut state, mut inventory, selected_recipe) in query {
        if !matches!(*state, ProcessState::Completed) {
            continue;
        }

        let Some(recipe) = recipes.get(&selected_recipe.0) else {
            continue;
        };

        for (item_id, amount) in recipe.output.iter() {
            inventory
                .items
                .entry(*item_id)
                .and_modify(|quantity| *quantity += amount)
                .or_insert(*amount);
        }

        *state = ProcessState::InsufficientInput;
    }
}
