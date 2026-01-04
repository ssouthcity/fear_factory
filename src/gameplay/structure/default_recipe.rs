use bevy::prelude::*;

use crate::gameplay::{
    recipe::select::SelectRecipe,
    structure::{Structure, assets::StructureDef, interactable::Interact},
    tome::inspect::Inspect,
    world::construction::StructureConstructed,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_default_recipe.run_if(on_message::<StructureConstructed>),
    );
}

fn apply_default_recipe(
    mut structures_constructed: MessageReader<StructureConstructed>,
    mut commands: Commands,
    structure_query: Query<&Structure>,
    structure_defs: Res<Assets<StructureDef>>,
) {
    for StructureConstructed(entity) in structures_constructed.read() {
        let Ok(Structure(handle)) = structure_query.get(*entity) else {
            continue;
        };

        let Some(structure_def) = structure_defs.get(handle) else {
            continue;
        };

        if let Some(recipe) = structure_def.default_recipe {
            commands.trigger(SelectRecipe {
                entity: *entity,
                recipe,
            });
        }

        commands
            .entity(*entity)
            .observe(|interact: On<Interact>, mut commands: Commands| {
                commands.trigger(Inspect {
                    entity: interact.entity,
                });
            });
    }
}
