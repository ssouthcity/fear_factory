use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    item::inventory::Inventory,
    recipe::{assets::Recipe, select::SelectedRecipe},
};

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
pub enum ProcessState {
    #[default]
    InsufficientInput,
    Working(Timer),
    Completed,
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

        if !can_afford_recipe(recipe, &inventory) {
            continue;
        }

        consume_recipe_input(recipe, &mut inventory);

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

        produce_recipe_output(recipe, &mut inventory);

        *state = ProcessState::InsufficientInput;
    }
}

fn can_afford_recipe(recipe: &Recipe, inventory: &Inventory) -> bool {
    recipe.input.iter().all(|(item_id, required_amount)| {
        inventory
            .items
            .get(item_id)
            .is_some_and(|quantity| quantity >= required_amount)
    })
}

fn consume_recipe_input(recipe: &Recipe, inventory: &mut Inventory) {
    for (item_id, required_amount) in recipe.input.iter() {
        inventory
            .items
            .entry(*item_id)
            .and_modify(|quantity| *quantity -= required_amount);
    }
}

fn produce_recipe_output(recipe: &Recipe, inventory: &mut Inventory) {
    for (item_id, amount) in recipe.output.iter() {
        inventory
            .items
            .entry(*item_id)
            .and_modify(|quantity| *quantity += amount)
            .or_insert(*amount);
    }
}
