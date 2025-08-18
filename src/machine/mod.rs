use bevy::prelude::*;

use crate::{
    FactorySystems,
    assets::manifest::{Id, Manifest},
    item::{Item, ItemAssets, Recipe, RecipeAssets, SelectedRecipe, Stack},
    logistics::{InputInventory, OutputInventory},
    machine::power::Powered,
};

mod assets;
mod build;
pub mod power;
pub mod progress;

pub use self::{
    assets::StructureTemplate,
    build::{Preview, QueueStructureSpawn},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Structure>();
    app.register_type::<Machine>();

    app.add_plugins((
        assets::plugin,
        build::plugin,
        power::plugin,
        progress::plugin,
    ));

    app.register_type::<WorkState>();
    app.add_systems(
        Update,
        (consume_input, progress_work, produce_output)
            .chain()
            .in_set(FactorySystems::Work),
    );
    app.add_systems(Update, instant_movers);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Structure(Id<StructureTemplate>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(WorkState)]
pub struct Machine;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = progress::on_work_state_add)]
pub enum WorkState {
    #[default]
    InsufficientInput,
    Working(Timer),
    Completed(Vec<Stack>),

    // Exceptions
    PerpetualWorker,
}

impl WorkState {
    pub fn is_working(&self) -> bool {
        matches!(self, Self::Working(_) | Self::PerpetualWorker)
    }

    pub fn progress(&self) -> f32 {
        match self {
            Self::InsufficientInput => 0.0,
            Self::Working(timer) => timer.fraction(),
            Self::Completed(_) => 1.0,
            _ => 1.0,
        }
    }
}

fn consume_input(
    query: Query<(&mut WorkState, &mut InputInventory, &SelectedRecipe), With<Powered>>,
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    recipe_assets: Res<RecipeAssets>,
) {
    for (mut state, mut inventory, selected_recipe) in query {
        if !matches!(*state, WorkState::InsufficientInput) {
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

        *state = WorkState::Working(timer);
    }
}

fn progress_work(
    query: Query<(&mut WorkState, &SelectedRecipe), With<Powered>>,
    recipe_manifests: Res<Assets<Manifest<Recipe>>>,
    recipe_assets: Res<RecipeAssets>,
    item_manifests: Res<Assets<Manifest<Item>>>,
    item_assets: Res<ItemAssets>,
    time: Res<Time>,
) {
    for (mut state, selected_recipe) in query {
        let WorkState::Working(ref mut timer) = *state else {
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

        *state = WorkState::Completed(output);
    }
}

fn produce_output(query: Query<(&mut WorkState, &mut OutputInventory), With<Powered>>) {
    for (mut state, mut output) in query {
        let WorkState::Completed(ref mut stacks) = *state else {
            continue;
        };

        let all_deposited = stacks
            .iter_mut()
            .all(|stack| output.add_stack(stack).is_ok());

        if all_deposited {
            *state = WorkState::InsufficientInput;
        }
    }
}

fn instant_movers(query: Query<(&WorkState, &mut InputInventory, &mut OutputInventory)>) {
    for (state, mut input, mut output) in query {
        if !matches!(state, WorkState::PerpetualWorker) {
            continue;
        }

        let _ = input.transfer_all(&mut output);
    }
}
