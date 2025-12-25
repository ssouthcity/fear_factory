use bevy::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        FactorySystems,
        item::{assets::ItemDef, inventory::Inventory},
        recipe::{assets::RecipeDef, select::SelectedRecipe},
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
    query: Query<(&mut ProcessState, &mut Inventory, &SelectedRecipe)>,
    recipes: Res<Assets<RecipeDef>>,
    item_index: Res<IndexMap<ItemDef>>,
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
                .get(item_index.get(item_id).unwrap())
                .is_some_and(|quantity| quantity >= required_amount)
        }) {
            continue;
        }

        for (raw_item_id, required_amount) in recipe.input.iter() {
            let item_id = item_index.get(raw_item_id).unwrap();
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
    recipes: Res<Assets<RecipeDef>>,
    item_index: Res<IndexMap<ItemDef>>,
) {
    for (mut state, mut inventory, selected_recipe) in query {
        if !matches!(*state, ProcessState::Completed) {
            continue;
        }

        let Some(recipe) = recipes.get(&selected_recipe.0) else {
            continue;
        };

        for (raw_item_id, amount) in recipe.output.iter() {
            let Some(item_id) = item_index.get(raw_item_id) else {
                continue;
            };

            inventory
                .items
                .entry(*item_id)
                .and_modify(|quantity| *quantity += amount)
                .or_insert(*amount);
        }

        *state = ProcessState::InsufficientInput;
    }
}
