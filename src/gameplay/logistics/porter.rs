use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::gameplay::{
    FactorySystems,
    item::{Item, Quantity},
    recipe::{InputOf, OutputOf},
    world::terrain::Worldly,
    y_sort::YSort,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Porters>();
    app.register_type::<PorterOf>();
    app.register_type::<PorterToStructure>();
    app.register_type::<PorterToInput>();
    app.register_type::<PorterArrival>();

    app.add_event::<PorterArrival>();

    app.add_systems(
        Update,
        (
            spawn_porter,
            set_porter_destination,
            move_towards_destination,
            drop_of_items,
        )
            .chain()
            .in_set(FactorySystems::Logistics),
    );
}

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
pub struct PorterToStructure(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PorterToInput(pub Entity);

#[derive(Event, Reflect)]
pub struct PorterArrival(pub Entity);

fn spawn_porter(
    output_query: Query<(&Item, &mut Quantity, &OutputOf)>,
    transform_query: Query<&Transform>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (item, mut quantity, OutputOf(structure)) in output_query {
        if quantity.0 == 0 {
            continue;
        }

        quantity.0 -= 1;

        let transform = transform_query.get(*structure).unwrap();

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

fn set_porter_destination(
    porter_query: Query<(Entity, &Item), Added<PorterOf>>,
    input_query: Query<(Entity, &Item, &InputOf)>,
    mut commands: Commands,
) {
    let mut rng = rand::rng();

    for (entity, item) in porter_query {
        let input = input_query
            .iter()
            .filter(|(_, i, _)| i.0 == item.0)
            .choose(&mut rng);

        if let Some((input_entity, _, InputOf(structure))) = input {
            commands
                .entity(entity)
                .insert((PorterToInput(input_entity), PorterToStructure(*structure)));
        }
    }
}

fn move_towards_destination(
    porter_query: Query<(Entity, &mut Transform, &PorterToStructure)>,
    transform_query: Query<&Transform, Without<PorterToStructure>>,
    time: Res<Time>,
    mut events: EventWriter<PorterArrival>,
) {
    const SPEED: f32 = 32.0;
    const ARRIVAL_THRESHHOLD: f32 = 32.0;

    for (porter, mut transform, porter_to) in porter_query {
        let Ok(goal_transform) = transform_query.get(porter_to.0) else {
            continue;
        };

        transform.translation = transform
            .translation
            .move_towards(goal_transform.translation, SPEED * time.delta_secs());

        if transform.translation.distance(goal_transform.translation) <= ARRIVAL_THRESHHOLD {
            events.write(PorterArrival(porter));
        }
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
