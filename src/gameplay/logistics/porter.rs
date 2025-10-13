use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

use crate::gameplay::{
    FactorySystems,
    item::stack::Stack,
    logistics::pathfinding::{PorterPaths, WalkPath},
    recipe::Outputs,
    sprite_sort::{YSortSprite, ZIndexSprite},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PorterArrival>();
    app.add_message::<PorterLost>();

    app.add_systems(
        Update,
        (spawn_porter, despawn_lost_porters, drop_of_items).in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
#[require(PorterSpawnOutputIndex)]
pub struct PorterSpawnTimer(pub Timer);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
struct PorterSpawnOutputIndex(usize);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = PorterOf)]
pub struct Porters(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Porters)]
pub struct PorterOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PorterFromOutput(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PorterToInput(pub Entity);

#[derive(Message, Reflect, Debug)]
pub struct PorterArrival(pub Entity);

#[derive(Message, Reflect, Debug)]
pub struct PorterLost(pub Entity);

fn spawn_porter(
    structure_query: Query<(
        Entity,
        &Transform,
        &mut PorterSpawnTimer,
        &mut PorterSpawnOutputIndex,
        &Outputs,
    )>,
    mut output_query: Query<(&mut Stack, &mut PorterPaths)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (structure, transform, mut timer, mut index, outputs) in structure_query {
        if !timer.tick(time.delta()).is_finished() {
            continue;
        }

        let Some(output) = outputs.iter().nth(index.0) else {
            continue;
        };

        let Ok((mut stack, mut porter_paths)) = output_query.get_mut(output) else {
            continue;
        };

        if stack.quantity == 0 {
            continue;
        }

        let Some((input, path)) = porter_paths.0.front() else {
            continue;
        };

        commands.spawn((
            Name::new("Porter"),
            *transform,
            Sprite::default(),
            Anchor(Vec2::new(0.0, -0.25)),
            AseAnimation {
                aseprite: asset_server.load("sprites/logistics/porter.aseprite"),
                animation: Animation::tag("walk"),
            },
            YSortSprite,
            ZIndexSprite(10),
            stack.clone(),
            PorterOf(structure),
            PorterFromOutput(output),
            PorterToInput(*input),
            WalkPath(path.clone()),
        ));

        stack.quantity -= 1;
        index.0 = (index.0 + 1) % outputs.len();
        porter_paths.0.rotate_left(1);
        timer.reset();
    }
}

fn despawn_lost_porters(mut porter_losses: MessageReader<PorterLost>, mut commands: Commands) {
    for PorterLost(entity) in porter_losses.read() {
        commands.entity(*entity).despawn();
    }
}

fn drop_of_items(
    mut poter_arrivals: MessageReader<PorterArrival>,
    porter_query: Query<&PorterToInput>,
    mut input_query: Query<&mut Stack>,
    mut commands: Commands,
) {
    for PorterArrival(porter) in poter_arrivals.read() {
        commands.entity(*porter).despawn();

        let PorterToInput(input) = porter_query.get(*porter).unwrap();

        if let Ok(mut stack) = input_query.get_mut(*input) {
            stack.quantity += 1;
        }
    }
}
