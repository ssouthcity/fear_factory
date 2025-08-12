use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use super::HIGHLIGHT_COLOR;

const INTERACTABLE_BUTTON: KeyCode = KeyCode::KeyE;

pub fn plugin(app: &mut App) {
    app.register_type::<Interactable>();
    app.register_type::<Interact>();

    app.register_type::<HoveredInteractable>();
    app.register_type::<ButtonIndicator>();
    app.register_type::<ButtonIndicatorOf>();

    app.init_resource::<HoveredInteractable>();
    app.init_resource::<InteractionAssets>();

    app.add_observer(on_hover);
    app.add_observer(on_leave);

    app.add_systems(Update, on_interact_button_click);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Interactable;

#[derive(Event, Reflect, Default)]
pub struct Interact;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct HoveredInteractable(Option<Entity>);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct InteractionAssets(Handle<Aseprite>);

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("alphabet-prompts.aseprite"))
    }
}

impl InteractionAssets {
    fn sprite(&self, key: KeyCode) -> impl Bundle {
        (
            Sprite::sized(Vec2::splat(8.0)),
            AseSlice {
                aseprite: self.0.clone(),
                name: match key {
                    KeyCode::KeyE => "E",
                    _ => "?",
                }
                .to_string(),
            },
        )
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = ButtonIndicatorOf, linked_spawn)]
struct ButtonIndicator(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ButtonIndicator)]
struct ButtonIndicatorOf(Entity);

fn on_hover(
    mut trigger: Trigger<Pointer<Over>>,
    interactables: Query<Entity, With<Interactable>>,
    mut commands: Commands,
    mut hovered_interactable: ResMut<HoveredInteractable>,
    interaction_assets: Res<InteractionAssets>,
) {
    if !interactables.contains(trigger.target) {
        return;
    }

    hovered_interactable.0 = Some(trigger.target);

    commands.spawn((
        Name::new("Interact Indicator"),
        Transform::from_xyz(0.0, 0.0, 1.0),
        ChildOf(trigger.target),
        ButtonIndicatorOf(trigger.target),
        interaction_assets.sprite(INTERACTABLE_BUTTON),
    ));

    trigger.propagate(false);
}

fn on_leave(
    mut trigger: Trigger<Pointer<Out>>,
    interactables: Query<Entity, With<Interactable>>,
    mut commands: Commands,
    mut hovered_interactable: ResMut<HoveredInteractable>,
) {
    if !interactables.contains(trigger.target) {
        return;
    }

    hovered_interactable.0 = None;

    commands
        .entity(trigger.target)
        .despawn_related::<ButtonIndicator>();

    trigger.propagate(false);
}

fn on_interact_button_click(
    keys: Res<ButtonInput<KeyCode>>,
    hovered_interactable: Res<HoveredInteractable>,
    mut commands: Commands,
) {
    if !keys.just_pressed(INTERACTABLE_BUTTON) {
        return;
    }

    let Some(interactable) = hovered_interactable.0 else {
        return;
    };

    commands.trigger_targets(Interact, interactable);
}
