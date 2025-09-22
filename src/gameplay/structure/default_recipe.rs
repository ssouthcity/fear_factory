use bevy::prelude::*;

use crate::gameplay::{
    hud::inspect::Inspect,
    recipe::select::SelectRecipe,
    structure::{Structure, assets::StructureDef, interactable::Interact},
    world::construction::StructureConstructed,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_default_recipe.run_if(on_event::<StructureConstructed>),
    );
}

fn apply_default_recipe(
    mut events: EventReader<StructureConstructed>,
    mut commands: Commands,
    structure_query: Query<&Structure>,
    structure_defs: Res<Assets<StructureDef>>,
) {
    for StructureConstructed(entity) in events.read() {
        let Ok(Structure(handle)) = structure_query.get(*entity) else {
            continue;
        };

        let Some(structure_def) = structure_defs.get(handle) else {
            continue;
        };

        if let Some(recipe) = &structure_def.default_recipe {
            commands
                .entity(*entity)
                .trigger(SelectRecipe(recipe.to_owned()));
        }

        commands
            .entity(*entity)
            .observe(|trigger: Trigger<Interact>, mut commands: Commands| {
                commands.trigger_targets(Inspect, trigger.target());
            });
    }
}
