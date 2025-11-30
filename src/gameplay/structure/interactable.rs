use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    assets::tracking::LoadResource,
    input::input_map::{Action, InputMap, action_just_pressed},
};

pub fn plugin(app: &mut App) {
    app.load_resource::<InteractionAssets>();

    app.init_resource::<HoveredInteractable>();

    app.add_observer(on_hover);
    app.add_observer(on_leave);

    app.add_systems(
        Update,
        on_interact_button_click.run_if(action_just_pressed(Action::Interact)),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Pickable)]
pub struct Interactable;

#[derive(EntityEvent, Reflect, Debug)]
pub struct Interact {
    pub entity: Entity,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct HoveredInteractable(Option<Entity>);

#[derive(Asset, Resource, Reflect, Clone)]
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
    mut pointer_over: On<Pointer<Over>>,
    interactables: Query<Entity, With<Interactable>>,
    mut commands: Commands,
    mut hovered_interactable: ResMut<HoveredInteractable>,
    interaction_assets: Res<InteractionAssets>,
    input_map: Res<InputMap>,
) {
    if !interactables.contains(pointer_over.entity) {
        return;
    }

    hovered_interactable.0 = Some(pointer_over.entity);

    commands.spawn((
        Name::new("Interact Indicator"),
        Transform::from_xyz(0.0, 0.0, 1.0),
        ChildOf(pointer_over.entity),
        ButtonIndicatorOf(pointer_over.entity),
        interaction_assets.sprite(*input_map.keymap.get(&Action::Interact).unwrap()),
    ));

    pointer_over.propagate(false);
}

fn on_leave(
    mut pointer_out: On<Pointer<Out>>,
    interactables: Query<Entity, With<Interactable>>,
    mut commands: Commands,
    mut hovered_interactable: ResMut<HoveredInteractable>,
) {
    if !interactables.contains(pointer_out.entity) {
        return;
    }

    hovered_interactable.0 = None;

    commands
        .entity(pointer_out.entity)
        .despawn_related::<ButtonIndicator>();

    pointer_out.propagate(false);
}

fn on_interact_button_click(
    hovered_interactable: Res<HoveredInteractable>,
    mut commands: Commands,
) {
    let Some(interactable) = hovered_interactable.0 else {
        return;
    };

    commands.trigger(Interact {
        entity: interactable,
    });
}
