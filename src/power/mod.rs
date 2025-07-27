use bevy::prelude::*;

use crate::machine::power::Powered;

pub mod grid;
pub mod pole;

pub fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, pole::plugin));

    // resources
    app.register_type::<CurrentPower>();
    // components
    app.register_type::<PowerProducer>();
    app.register_type::<PowerConsumer>();
    // events
    app.register_type::<FuseBroke>();

    app.init_resource::<CurrentPower>();

    app.add_systems(FixedUpdate, (produce_power, consume_power).chain());

    // debug
    app.register_type::<CurrentPowerUI>();
    app.add_systems(Startup, spawn_debug_ui);
    app.add_systems(Update, update_debug_ui.after(consume_power));
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct CurrentPower(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerProducer(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerConsumer(pub f32);

#[derive(Event, Reflect)]
pub struct FuseBroke;

fn produce_power(
    power_producers: Query<&PowerProducer, With<Powered>>,
    mut current_power: ResMut<CurrentPower>,
    time: Res<Time>,
) {
    current_power.0 = power_producers
        .iter()
        .map(|f| f.0 * time.delta_secs())
        .sum();
}

fn consume_power(
    mut commands: Commands,
    power_consumers: Query<&PowerConsumer, With<Powered>>,
    mut current_power: ResMut<CurrentPower>,
    time: Res<Time>,
) {
    for consumer in power_consumers {
        current_power.0 -= consumer.0 * time.delta_secs();

        if current_power.0 < 0.0 {
            commands.trigger(FuseBroke);
            break;
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CurrentPowerUI;

fn spawn_debug_ui(mut commands: Commands) {
    commands.spawn((
        CurrentPowerUI,
        Text::default(),
        TextColor::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::ZERO,
            left: Val::ZERO,
            ..default()
        },
    ));
}

fn update_debug_ui(
    current_power: Res<CurrentPower>,
    mut text: Single<&mut Text, With<CurrentPowerUI>>,
) {
    text.0 = (current_power.0 * 64.0).to_string();
}
