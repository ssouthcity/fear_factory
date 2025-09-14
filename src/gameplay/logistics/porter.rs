use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    item::{Item, Quantity},
    recipe::OutputOf,
    world::terrain::Worldly,
    y_sort::YSort,
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
pub struct PorterSpawnTimer(pub Timer);

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
pub struct PorterToInput(pub Entity);

#[derive(Event, Reflect)]
pub struct PorterArrival(pub Entity);

fn spawn_porter(
    mut structure_query: Query<(&Transform, &mut PorterSpawnTimer)>,
    output_query: Query<(&Item, &mut Quantity, &OutputOf)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (item, mut quantity, OutputOf(structure)) in output_query {
        if quantity.0 == 0 {
            continue;
        }

        let Ok((transform, mut porter_spawn_timer)) = structure_query.get_mut(*structure) else {
            continue;
        };

        if !porter_spawn_timer.tick(time.delta()).just_finished() {
            continue;
        }

        quantity.0 -= 1;

        commands.spawn((
            Name::new("Porter"),
            *transform,
            Worldly,
            Sprite::from_image(asset_server.load("sprites/logistics/porter.png")),
            YSort::default(),
            item.clone(),
            PorterOf(*structure),
        ));
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
