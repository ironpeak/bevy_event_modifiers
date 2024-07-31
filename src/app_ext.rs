use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub trait EventModifiersAppExt {
    fn add_event_with_modifiers<E: Event>(&mut self) -> &mut Self;
}

impl EventModifiersAppExt for App {
    fn add_event_with_modifiers<E: Event>(&mut self) -> &mut Self {
        todo!()
    }
}
