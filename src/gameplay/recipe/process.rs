use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    inventory::prelude::*,
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
    query: Query<(Entity, &mut ProcessState, &SelectedRecipe)>,
    recipes: Res<Assets<Recipe>>,
    inventory: Query<&Inventory>,
    mut input_stacks: Query<(&mut ItemStack, &Input)>,
) {
    for (entity, mut state, selected_recipe) in query {
        if !matches!(*state, ProcessState::InsufficientInput) {
            continue;
        }

        let Some(recipe) = recipes.get(&selected_recipe.0) else {
            continue;
        };

        if !can_afford_recipe(entity, &inventory, &input_stacks) {
            continue;
        }

        consume_recipe_input(entity, &inventory, &mut input_stacks);

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
    query: Query<(Entity, &mut ProcessState)>,
    inventory: Query<&Inventory>,
    mut output_stacks: Query<(&mut ItemStack, &Output)>,
) {
    for (entity, mut state) in query {
        if !matches!(*state, ProcessState::Completed) {
            continue;
        }

        produce_recipe_output(entity, &inventory, &mut output_stacks);

        *state = ProcessState::InsufficientInput;
    }
}

fn can_afford_recipe(
    entity: Entity,
    inventory: &Query<&Inventory>,
    stacks: &Query<(&mut ItemStack, &Input)>,
) -> bool {
    inventory
        .iter_descendants(entity)
        .filter_map(|e| stacks.get(e).ok())
        .all(|(stack, input)| stack.quantity >= input.requirement)
}

fn consume_recipe_input(
    entity: Entity,
    inventory: &Query<&Inventory>,
    stacks: &mut Query<(&mut ItemStack, &Input)>,
) {
    for slot in inventory.iter_descendants(entity) {
        if let Ok((mut stack, input)) = stacks.get_mut(slot) {
            stack.quantity -= input.requirement;
        }
    }
}

fn produce_recipe_output(
    entity: Entity,
    inventory: &Query<&Inventory>,
    stacks: &mut Query<(&mut ItemStack, &Output)>,
) {
    for slot in inventory.iter_descendants(entity) {
        if let Ok((mut stack, output)) = stacks.get_mut(slot) {
            stack.quantity += output.production;
        }
    }
}
