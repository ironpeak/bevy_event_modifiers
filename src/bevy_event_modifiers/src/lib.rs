extern crate proc_macro;

pub(crate) mod app_ext;
pub(crate) mod context;

pub mod prelude {
    pub use bevy_event_modifiers_macros::EventModifierContext;

    pub use crate::{app_ext::EventModifiersAppExt, context::EventModifierContext};
}
