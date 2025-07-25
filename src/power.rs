use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    // resources
    app.register_type::<CurrentPower>();
    app.register_type::<FuseStatus>();
    // components
    app.register_type::<PowerProducer>();
    app.register_type::<PowerConsumer>();
    // events
    app.register_type::<BrokenFuse>();

    app.init_resource::<CurrentPower>();
    app.init_resource::<FuseStatus>();

    app.add_systems(
        FixedUpdate,
        (produce_power, consume_power)
            .chain()
            .run_if(not(fuse_is_broken)),
    );

    app.add_observer(handle_broken_fuse);

    app.add_systems(Update, fix_fuse);

    // debug
    app.register_type::<CurrentPowerUI>();
    app.add_systems(Startup, spawn_debug_ui);
    app.add_systems(Update, update_debug_ui);
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct CurrentPower(pub f32);

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct FuseStatus(pub bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerProducer(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerConsumer(pub f32);

#[derive(Event, Reflect)]
pub struct BrokenFuse;

pub fn fuse_is_broken(fuse_status: Res<FuseStatus>) -> bool {
    return fuse_status.0;
}

fn fix_fuse(keys: Res<ButtonInput<KeyCode>>, mut fuse_status: ResMut<FuseStatus>) {
    if keys.just_pressed(KeyCode::KeyF) {
        fuse_status.0 = false;
    }
}

fn produce_power(
    power_producers: Query<&PowerProducer>,
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
    power_consumers: Query<&PowerConsumer>,
    mut current_power: ResMut<CurrentPower>,
    time: Res<Time>,
) {
    for consumer in power_consumers {
        current_power.0 -= consumer.0 * time.delta_secs();

        if current_power.0 < 0.0 {
            commands.trigger(BrokenFuse);
            break;
        }
    }
}

fn handle_broken_fuse(_trigger: Trigger<BrokenFuse>, mut fuse_status: ResMut<FuseStatus>) {
    fuse_status.0 = true;
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
            right: Val::ZERO,
            ..default()
        },
    ));
}

fn update_debug_ui(
    current_power: Res<CurrentPower>,
    fuse_status: Res<FuseStatus>,
    mut text: Single<&mut Text, With<CurrentPowerUI>>,
    mut text_color: Single<&mut TextColor, With<CurrentPowerUI>>,
) {
    text.0 = current_power.0.to_string();

    text_color.0 = if fuse_status.0 {
        Color::linear_rgb(1.0, 0.0, 0.0)
    } else {
        Color::default()
    };
}
