use bevy::prelude::*;

use crate::simulation::{
    FactorySystems,
    item::{ItemDef, Stack},
    logistics::{InputInventory, OutputInventory},
    machine::power::Powered,
    recipe::RecipeDef,
};

use super::{SelectedRecipe, progress::on_progress_state_add};

pub fn plugin(app: &mut App) {
    app.register_type::<ProcessState>();

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
    Completed(Vec<Stack>),
}

impl ProcessState {
    pub fn progress(&self) -> f32 {
        match self {
            Self::InsufficientInput => 0.0,
            Self::Working(timer) => timer.fraction(),
            Self::Completed(_) => 1.0,
        }
    }
}

fn consume_input(
    query: Query<(&mut ProcessState, &mut InputInventory, &SelectedRecipe), With<Powered>>,
    recipes: Res<Assets<RecipeDef>>,
) {
    for (mut state, mut inventory, selected_recipe) in query {
        if !matches!(*state, ProcessState::InsufficientInput) {
            continue;
        }

        let Some(ref recipe_id) = selected_recipe.0 else {
            continue;
        };

        let recipe = recipes
            .iter()
            .map(|(_, recipe_def)| recipe_def)
            .find(|recipe_def| recipe_def.id == *recipe_id)
            .expect("Selected recipe refers to non-existent recipe");

        if inventory.consume_input(recipe).is_err() {
            continue;
        }

        let timer = Timer::new(recipe.duration, TimerMode::Once);

        *state = ProcessState::Working(timer);
    }
}

fn progress_work(
    query: Query<(&mut ProcessState, &SelectedRecipe), With<Powered>>,
    recipes: Res<Assets<RecipeDef>>,
    items: Res<Assets<ItemDef>>,
    time: Res<Time>,
) {
    for (mut state, selected_recipe) in query {
        let ProcessState::Working(ref mut timer) = *state else {
            continue;
        };

        if !timer.tick(time.delta()).finished() {
            continue;
        }

        let Some(ref recipe_id) = selected_recipe.0 else {
            continue;
        };

        let recipe = recipes
            .iter()
            .map(|(_, recipe_def)| recipe_def)
            .find(|recipe_def| recipe_def.id == *recipe_id)
            .expect("Selected recipe refers to non-existent id");

        let output: Vec<Stack> = recipe
            .output
            .iter()
            .map(|(item_id, quantity)| {
                let item_def = items
                    .iter()
                    .map(|(_, item_def)| item_def)
                    .find(|item_def| item_def.id == *item_id)
                    .expect("Recipe refers to non-existent output item");

                Stack {
                    item_id: item_def.id.to_owned(),
                    quantity: *quantity,
                    max_quantity: item_def.stack_size,
                }
            })
            .collect();

        *state = ProcessState::Completed(output);
    }
}

fn produce_output(query: Query<(&mut ProcessState, &mut OutputInventory), With<Powered>>) {
    for (mut state, mut output) in query {
        let ProcessState::Completed(ref mut stacks) = *state else {
            continue;
        };

        let all_deposited = stacks
            .iter_mut()
            .all(|stack| output.add_stack(stack).is_ok());

        if all_deposited {
            *state = ProcessState::InsufficientInput;
        }
    }
}
