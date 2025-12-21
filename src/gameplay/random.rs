use bevy::prelude::*;
use rand::{SeedableRng, rngs::StdRng};

pub fn plugin(app: &mut App) {
    app.init_resource::<Seed>();
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Seed(pub StdRng);

impl Default for Seed {
    fn default() -> Self {
        Self(StdRng::from_os_rng())
    }
}
