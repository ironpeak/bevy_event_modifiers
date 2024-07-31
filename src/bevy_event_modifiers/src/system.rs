use bevy_ecs::prelude::*;

use crate::prelude::*;

pub(crate) fn system<EventIn, ModifierPriority, ModifierContext, EventOut>(
    events_in: EventReader<EventIn>,
    modifiers: Query<&ModifierComponent<ModifierPriority, ModifierContext>>,
    events_out: EventWriter<EventOut>,
) where
    EventIn: Event,
    ModifierPriority: Eq + PartialEq + Ord + PartialOrd + Send + Sync + 'static,
    ModifierContext: Send + Sync + 'static,
    EventOut: Event,
{
    for modifier in modifiers
        .iter()
        .sort::<&ModifierComponent<ModifierPriority, ModifierContext>>()
    {}
}
