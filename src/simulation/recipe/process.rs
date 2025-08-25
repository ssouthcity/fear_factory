use bevy::prelude::*;

use crate::{
    assets::manifest::Manifest,
    simulation::{
        FactorySystems,
        item::{Item, ItemAssets, Stack},
        logistics::{InputInventory, OutputInventory},
        machine::power::Powered,
    },
};

use super::{Recipe, RecipeAssets, SelectedRecipe, progress::on_progress_state_add};

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
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    recipe_assets: Res<RecipeAssets>,
) {
    for (mut state, mut inventory, selected_recipe) in query {
        if !matches!(*state, ProcessState::InsufficientInput) {
            continue;
        }

        let Some(ref recipe_id) = selected_recipe.0 else {
            continue;
        };

        let recipe = recipe_manifests
            .get(&recipe_assets.manifest)
            .expect("Recipe manifest is not loaded")
            .get(recipe_id)
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
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    recipe_assets: Res<RecipeAssets>,
    item_manifests: Res<Assets<Manifest<Item>>>,
    item_assets: Res<ItemAssets>,
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

        let recipe = recipe_manifests
            .get(&recipe_assets.manifest)
            .expect("Recipe manifest not loaded")
            .get(recipe_id)
            .expect("Selected recipe refers to non-existent id");

        let items = item_manifests
            .get(&item_assets.manifest)
            .expect("Item manifest not loaded");

        let output: Vec<Stack> = recipe
            .output
            .iter()
            .map(|(item_id, quantity)| {
                let item = items
                    .get(item_id)
                    .expect("Recipe refers to non-existent item");

                Stack::from(item).with_quantity(*quantity)
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
