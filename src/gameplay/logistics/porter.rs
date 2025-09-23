use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    item::{Item, Quantity},
    logistics::pathfinding::{PorterPaths, WalkPath},
    recipe::Outputs,
    sprite_sort::YSortSprite,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Porters>();
    app.register_type::<PorterOf>();
    app.register_type::<PorterToInput>();
    app.register_type::<PorterArrival>();

    app.add_event::<PorterArrival>();

    app.add_systems(
        Update,
        (spawn_porter, drop_of_items).in_set(FactorySystems::Logistics),
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

#[derive(Event, Reflect)]
pub struct PorterArrival(pub Entity);

fn spawn_porter(
    structure_query: Query<(
        Entity,
        &Transform,
        &mut PorterSpawnTimer,
        &mut PorterSpawnOutputIndex,
        &Outputs,
    )>,
    mut output_query: Query<(&Item, &mut Quantity, &mut PorterPaths)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (structure, transform, mut timer, mut index, outputs) in structure_query {
        if !timer.tick(time.delta()).finished() {
            continue;
        }

        let Some(output) = outputs.iter().nth(index.0) else {
            continue;
        };

        let Ok((item, mut quantity, mut porter_paths)) = output_query.get_mut(output) else {
            continue;
        };

        if quantity.0 == 0 {
            continue;
        }

        let Some((input, path)) = porter_paths.0.front() else {
            continue;
        };

        commands.spawn((
            Name::new("Porter"),
            *transform,
            Sprite::from_image(asset_server.load("sprites/logistics/porter.png")),
            YSortSprite,
            item.clone(),
            PorterOf(structure),
            PorterFromOutput(output),
            PorterToInput(*input),
            WalkPath(path.clone()),
        ));

        quantity.0 -= 1;
        index.0 = (index.0 + 1) % outputs.len();
        porter_paths.0.rotate_left(1);
        timer.reset();
    }
}

fn drop_of_items(
    mut events: EventReader<PorterArrival>,
    porter_query: Query<&PorterToInput>,
    mut input_query: Query<&mut Quantity>,
    mut commands: Commands,
) {
    for PorterArrival(porter) in events.read() {
        commands.entity(*porter).despawn();

        let PorterToInput(input) = porter_query.get(*porter).unwrap();

        if let Ok(mut quantity) = input_query.get_mut(*input) {
            quantity.0 += 1;
        }
    }
}
