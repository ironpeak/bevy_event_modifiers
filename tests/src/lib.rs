use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_event_modifiers::prelude::*;
use bevy_event_modifiers_macros::EventModifier;

#[derive(Event)]
pub struct CombatEventInput;

pub enum CombatEventModifierPriority {
    Low,
    Medium,
    High,
}

pub struct CombatEventModifierContext;

#[derive(Event)]
pub struct CombatEventOutput;

#[derive(EventModifier)]
pub struct CombatEvent<'a, 'b, 'c> {
    pub input: CombatEventInput,
    pub priority: CombatEventModifierPriority,
    pub context: &'a (Query<'b, 'c, Entity>),
    pub output: CombatEventOutput,
}

pub fn init(app: &mut App) {
    app.add_event_with_modifiers::<CombatEvent<'_, '_, '_>>();
}
