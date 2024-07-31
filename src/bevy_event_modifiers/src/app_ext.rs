use bevy_app::prelude::*;

use crate::prelude::EventModifier;

pub trait EventModifiersAppExt {
    fn add_event_with_modifiers<T>(&mut self) -> &mut Self
    where
        T: EventModifier;
}

impl EventModifiersAppExt for App {
    fn add_event_with_modifiers<T>(&mut self) -> &mut Self
    where
        T: EventModifier,
    {
        T::register_type(self);
        self
    }
}
