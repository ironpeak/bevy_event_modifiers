use crate::prelude::*;

pub mod prelude {
    pub(crate) use bevy_app::prelude::*;
    pub(crate) use bevy_ecs::prelude::*;
    pub(crate) use bevy_event_modifiers::prelude::*;
    pub(crate) use rand::{rngs::StdRng, RngCore, SeedableRng};

    pub(crate) use crate::{Armor, CriticalChance, InvulnerabilityEvent, Invulnerable, Rng};
}

mod attack_event;

#[derive(Resource)]
pub(crate) struct Rng {
    pub rng: StdRng,
}

#[derive(Component)]
pub(crate) struct Armor {
    pub value: u32,
}

#[derive(Component)]
pub(crate) struct CriticalChance {
    pub value: u32,
}

#[derive(Component)]
pub(crate) struct Invulnerable {}

#[derive(Event)]
pub(crate) struct InvulnerabilityEvent {}

pub fn init(app: &mut App) {
    attack_event::init(app);
}
