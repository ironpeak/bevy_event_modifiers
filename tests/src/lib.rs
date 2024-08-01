// use bevy_app::prelude::*;
// use bevy_ecs::prelude::*;
// use bevy_event_modifiers::prelude::*;
// use bevy_event_modifiers_macros::{EventModifier, EventModifierContext};

// #[derive(Component)]
// pub struct ExampleComponent;

// #[derive(Event)]
// pub struct CombatEventIn;

// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum CombatEventModifierPriority {
//     Low,
//     Medium,
//     High,
// }

// #[derive(EventModifierContext)]
// pub struct CombatEventModifierContext<'w, 's> {
//     pub entities:
//         &'s Query<'w, 's, (&'s ExampleComponent, &'s ExampleComponent)>,
// }

// #[derive(Event)]
// pub struct CombatEventOut;

// pub fn init(app: &mut App) {
//     // app.add_event_with_modifiers::<CombatEvent<'_, '_>>();
// }

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_event_modifiers::prelude::*;
use bevy_event_modifiers_macros::{EventModifier, EventModifierContext};

#[derive(Component)]
pub struct ExampleComponent;

#[derive(Resource)]
pub struct ExampleResource;

#[derive(Event)]
pub struct ExampleEvent;

#[derive(Event)]
pub struct CombatEventIn;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CombatEventModifierPriority {
    Low,
    Medium,
    High,
}

pub struct CombatEventModifierMetadata {}

impl CombatEventModifierMetadata {
    fn new(context: &mut CombatEventModifierContext, event: &CombatEventIn) -> Self {
        CombatEventModifierMetadata {}
    }
}

pub struct CombatEventModifierContext<'w, 's> {
    pub entities: Query<'w, 's, (&'static ExampleComponent, &'static ExampleComponent)>,
    pub resource_mut: ResMut<'w, ExampleResource>,
    pub resource: Res<'w, ExampleResource>,
    pub event_reader: EventReader<'w, 's, ExampleEvent>,
    pub event_writer: EventWriter<'w, ExampleEvent>,
}
pub struct CombatEventModifier {
    pub priority: CombatEventModifierPriority,
    pub modify:
        fn(&mut CombatEventModifierContext, &mut CombatEventModifierMetadata, &mut CombatEventOut),
}
impl bevy_ecs::component::Component for CombatEventModifier
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}
impl Ord for CombatEventModifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
impl PartialOrd for CombatEventModifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}
impl Eq for CombatEventModifier {}
impl PartialEq for CombatEventModifier {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
impl<'w, 's> CombatEventModifierContext<'w, 's> {
    fn system(
        mut p_events_in: bevy_ecs::prelude::EventReader<CombatEventIn>,
        entities: Query<(&'static ExampleComponent, &'static ExampleComponent)>,
        resource_mut: ResMut<ExampleResource>,
        resource: Res<ExampleResource>,
        event_reader: EventReader<ExampleEvent>,
        event_writer: EventWriter<ExampleEvent>,
        p_modifiers: bevy_ecs::prelude::Query<&CombatEventModifier>,
        mut p_events_out: bevy_ecs::prelude::EventWriter<CombatEventOut>,
    ) {
        let mut context = CombatEventModifierContext {
            entities,
            resource_mut,
            resource,
            event_reader,
            event_writer,
        };
        let modifiers = p_modifiers
            .iter()
            .sort::<&CombatEventModifier>()
            .collect::<Vec<_>>();
        for event in p_events_in.read() {
            let mut metadata = CombatEventModifierMetadata::new(&mut context, event);
            let mut event_out = CombatEventOut::new(&mut context, event);
            for modifier in &modifiers {
                (modifier.modify)(&mut context, &mut metadata, &mut event_out);
            }
            p_events_out.send(event_out);
        }
    }
}
impl<'w, 's> bevy_event_modifiers::prelude::EventModifierContext
    for CombatEventModifierContext<'w, 's>
{
    fn register_type(app: &mut bevy_app::prelude::App) -> &mut bevy_app::prelude::App {
        app.add_event::<CombatEventIn>();
        app.add_event::<CombatEventOut>();
        app.add_systems(
            bevy_app::prelude::Update,
            CombatEventModifierContext::system,
        );
        app
    }
}
pub struct CombatEventOut;
impl bevy_ecs::event::Event for CombatEventOut where Self: Send + Sync + 'static {}
impl bevy_ecs::component::Component for CombatEventOut
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: bevy_ecs::component::StorageType =
        bevy_ecs::component::StorageType::SparseSet;
}

impl CombatEventOut {
    fn new(context: &mut CombatEventModifierContext, event: &CombatEventIn) -> Self {
        CombatEventOut {}
    }
}

pub fn init(app: &mut App) {}
