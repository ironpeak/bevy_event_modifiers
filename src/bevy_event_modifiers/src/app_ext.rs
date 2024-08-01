use bevy_app::prelude::*;

use crate::prelude::EventModifierContext;

pub trait EventModifiersAppExt {
    fn add_event_with_modifiers<T>(&mut self) -> &mut Self
    where
        T: EventModifierContext;
}

impl EventModifiersAppExt for App {
    fn add_event_with_modifiers<T>(&mut self) -> &mut Self
    where
        T: EventModifierContext,
    {
        T::register_type(self);
        self
    }
}
