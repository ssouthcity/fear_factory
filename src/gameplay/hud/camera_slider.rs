use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{
        CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue, TrackClick,
        ValueChange, observe,
    },
};

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.init_resource::<CameraZoomLevel>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_camera_slider);

    app.add_systems(
        Update,
        (
            update_widget_values,
            (
                update_slider_style,
                update_slider_style2,
                update_camera_zoom,
            )
                .after(update_widget_values),
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct CameraZoomLevel {
    value: f32,
}

impl Default for CameraZoomLevel {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraSlider;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraSliderThumb;

fn spawn_camera_slider(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100.0),
            height: percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            padding: px(32.0).all(),
            ..default()
        },
        children![(
            slider(0.25, 4.0, 1.0),
            UiTransform {
                rotation: Rot2::degrees(90.0),
                translation: Val2::percent(50.0, 0.0),
                ..default()
            },
        )],
    ));
}

fn slider(min: f32, max: f32, base: f32) -> impl Bundle {
    (
        Name::new("Slider"),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: percent(30),
            ..default()
        },
        Hovered::default(),
        CameraSlider,
        Slider {
            track_click: TrackClick::Snap,
        },
        SliderValue(base),
        SliderRange::new(min, max),
        observe(
            |value_change: On<ValueChange<f32>>, mut camera_zoom_level: ResMut<CameraZoomLevel>| {
                let quarter = (value_change.value * 4.0).round() / 4.0;
                camera_zoom_level.value = quarter;
            },
        ),
        Children::spawn((
            Spawn((
                Node {
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            )),
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    CameraSliderThumb,
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                )],
            )),
        )),
    )
}

fn update_widget_values(
    res: Res<CameraZoomLevel>,
    mut sliders: Query<Entity, With<CameraSlider>>,
    mut commands: Commands,
) {
    if !res.is_changed() {
        return;
    }

    for slider in sliders.iter_mut() {
        commands.entity(slider).insert(SliderValue(res.value));
    }
}

fn update_slider_style(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &CoreSliderDragState,
            Has<InteractionDisabled>,
        ),
        (
            Or<(
                Changed<SliderValue>,
                Changed<SliderRange>,
                Changed<Hovered>,
                Changed<CoreSliderDragState>,
                Added<InteractionDisabled>,
            )>,
            With<CameraSlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<
        (&mut Node, &mut BackgroundColor, Has<CameraSliderThumb>),
        Without<CameraSlider>,
    >,
) {
    for (slider_ent, value, range, hovered, drag_state, disabled) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
                thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
            }
        }
    }
}

fn update_slider_style2(
    sliders: Query<
        (
            Entity,
            &Hovered,
            &CoreSliderDragState,
            Has<InteractionDisabled>,
        ),
        With<CameraSlider>,
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut BackgroundColor, Has<CameraSliderThumb>), Without<CameraSlider>>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
) {
    removed_disabled.read().for_each(|entity| {
        if let Ok((slider_ent, hovered, drag_state, disabled)) = sliders.get(entity) {
            for child in children.iter_descendants(slider_ent) {
                if let Ok((mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                    && is_thumb
                {
                    thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
                }
            }
        }
    });
}

const SLIDER_THUMB: Color = Color::hsl(60.0, 0.8, 0.5);
const ELEMENT_FILL_DISABLED: Color = Color::hsl(0.0, 0.1, 0.5);

fn thumb_color(disabled: bool, hovered: bool) -> Color {
    match (disabled, hovered) {
        (true, _) => ELEMENT_FILL_DISABLED,

        (false, true) => SLIDER_THUMB.lighter(0.3),

        _ => SLIDER_THUMB,
    }
}

fn update_camera_zoom(
    mut projection: Single<&mut Projection, With<Camera>>,
    camera_zoom_level: Res<CameraZoomLevel>,
) {
    if !camera_zoom_level.is_changed() {
        return;
    }

    let Projection::Orthographic(ref mut ortho) = **projection else {
        return;
    };

    ortho.scale = camera_zoom_level.value;
}
