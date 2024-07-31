extern crate proc_macro;

pub(crate) mod app_ext;
pub(crate) mod modifier;

pub mod prelude {
    pub use crate::{app_ext::EventModifiersAppExt, modifier::EventModifier};
}
