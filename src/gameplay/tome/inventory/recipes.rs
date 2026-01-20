use bevy::{
    prelude::*,
    ui_widgets::{RadioButton, RadioGroup, ValueChange, observe},
};

use crate::{
    gameplay::{
        inventory::prelude::*,
        recipe::assets::Recipe,
        tome::{UITomeLeftPageRoot, UITomeRightPageRoot, inventory::InventoryTabs, list_page},
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<FocusedRecipe>();

    app.add_systems(OnEnter(InventoryTabs::Recipes), spawn_recipe_list);

    app.add_systems(
        Update,
        spawn_recipe_details.run_if(state_changed::<FocusedRecipe>),
    );

    app.add_systems(Update, update_item_badge);
}

#[derive(SubStates, Default, Debug, Hash, PartialEq, Eq, Clone)]
#[source(InventoryTabs = InventoryTabs::Recipes)]
struct FocusedRecipe(Option<AssetId<Recipe>>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RecipeButton(AssetId<Recipe>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemBadge(AssetId<ItemDef>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemThumbnail;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ItemName;

fn item_badge(asset_id: AssetId<ItemDef>, quantity: u32) -> impl Bundle {
    (
        ItemBadge(asset_id),
        Node {
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::px(32.0),
                GridTrack::fr(1.0),
                GridTrack::px(32.0),
            ],
            padding: UiRect::axes(px(32.0), px(16.0)),
            column_gap: px(8.0),
            border: px(2.0).all(),
            ..default()
        },
        BorderColor::all(Color::BLACK),
        children![
            (
                ItemThumbnail,
                Node {
                    width: px(32.0),
                    height: px(32.0),
                    ..default()
                },
                ImageNode::default()
            ),
            (ItemName, Text::default()),
            (Text::new(format!("{quantity}"))),
        ],
    )
}

fn update_item_badge(
    items: Res<Assets<ItemDef>>,
    badges: Query<(Entity, &ItemBadge), Changed<ItemBadge>>,
    children: Query<&Children>,
    mut item_thumbnails: Query<&mut ImageNode, With<ItemThumbnail>>,
    mut item_names: Query<&mut Text, With<ItemName>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (badge, ItemBadge(id)) in badges {
        let Some(item) = items.get(*id) else {
            continue;
        };
        for child in children.iter_descendants(badge) {
            if let Ok(mut image_node) = item_thumbnails.get_mut(child)
                && let Some(image) = images.get_strong_handle(item.sprite)
            {
                image_node.image = image;
            }

            if let Ok(mut text_node) = item_names.get_mut(child) {
                text_node.0 = item.name.clone();
            }
        }
    }
}

fn spawn_recipe_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    recipes: Res<Assets<Recipe>>,
) {
    let recipe_list = commands
        .spawn((
            list_page(),
            ChildOf(*left_page),
            DespawnOnExit(InventoryTabs::Recipes),
            RadioGroup,
            observe(
                |value_change: On<ValueChange<Entity>>,
                 buttons: Query<&RecipeButton>,
                 mut next_focused_recipe: ResMut<NextState<FocusedRecipe>>| {
                    if let Ok(button) = buttons.get(value_change.value) {
                        next_focused_recipe.set(FocusedRecipe(Some(button.0)));
                    }
                },
            ),
        ))
        .id();

    for (asset_id, _) in recipes.iter() {
        commands.spawn((
            widgets::recipe_plate(asset_id),
            RadioButton,
            RecipeButton(asset_id),
            ChildOf(recipe_list),
        ));
    }
}

fn spawn_recipe_details(
    mut commands: Commands,
    right_page: Single<Entity, With<UITomeRightPageRoot>>,
    recipes: Res<Assets<Recipe>>,
    focused_recipe: Res<State<FocusedRecipe>>,
) {
    let FocusedRecipe(Some(recipe_id)) = focused_recipe.get() else {
        return;
    };

    let Some(recipe) = recipes.get(*recipe_id) else {
        return;
    };

    commands.spawn((
        list_page(),
        ChildOf(*right_page),
        DespawnOnExit(focused_recipe.get().clone()),
        Children::spawn((
            Spawn(Text::new(recipe.name.clone())),
            Spawn(Text::new(format!("{} seconds", recipe.duration.as_secs()))),
            Spawn(Text::new("Inputs")),
            SpawnIter(
                recipe
                    .input
                    .clone()
                    .into_iter()
                    .map(|(item, quantity)| item_badge(item, quantity)),
            ),
            Spawn(Text::new("Outputs")),
            SpawnIter(
                recipe
                    .output
                    .clone()
                    .into_iter()
                    .map(|(item, quantity)| item_badge(item, quantity)),
            ),
        )),
    ));
}
