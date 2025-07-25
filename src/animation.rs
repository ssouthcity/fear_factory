use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<AnimatedMachine>();

    app.add_plugins(AsepriteUltraPlugin);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(immutable, on_add = on_add_animated_machine)]
pub struct AnimatedMachine(pub &'static str);

fn on_add_animated_machine(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let Some(animated_machine) = world.entity(entity).get::<AnimatedMachine>() else {
        unreachable!(
            "AnimatedMachine component is guaranteed to be on entity because hook was invoked"
        );
    };

    let asset_server = world.resource::<AssetServer>();

    let aseprite = asset_server.load::<Aseprite>(animated_machine.0);

    world.commands().entity(entity).insert(AseAnimation {
        aseprite,
        animation: Animation::tag("work"),
    });
}
