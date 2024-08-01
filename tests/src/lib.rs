use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_event_modifiers::prelude::*;
use bevy_event_modifiers_macros::EventModifierContext;

#[derive(Component)]
pub struct ExampleComponent;

#[derive(Resource)]
pub struct ExampleResource;

#[derive(Event)]
pub struct ExampleEvent;

#[derive(Event)]
pub struct CombatEventIn;

#[derive(Event)]
pub struct CombatEventOut;

impl CombatEventOut {
    fn init(context: &mut CombatEventModifierContext, event: &CombatEventIn) -> Self {
        CombatEventOut {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CombatEventModifierPriority {
    Low,
    Medium,
    High,
}

pub struct CombatEventModifierMetadata {}

impl CombatEventModifierMetadata {
    fn init(context: &mut CombatEventModifierContext, event: &CombatEventIn) -> Self {
        CombatEventModifierMetadata {}
    }
}

#[derive(EventModifierContext)]
pub struct CombatEventModifierContext<'w, 's> {
    pub entities: Query<'w, 's, (&'static ExampleComponent, &'static ExampleComponent)>,
    pub resource_mut: ResMut<'w, ExampleResource>,
    pub resource: Res<'w, ExampleResource>,
    pub event_reader: EventReader<'w, 's, ExampleEvent>,
    pub event_writer: EventWriter<'w, ExampleEvent>,
}

pub fn init(app: &mut App) {
    app.add_event_with_modifiers::<CombatEventModifierContext<'_, '_>>();
}
