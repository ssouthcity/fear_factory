use bevy::prelude::*;

mod gameplay;
mod splash;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((gameplay::plugin, splash::plugin));
}

#[derive(States, Hash, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Gameplay,
}
