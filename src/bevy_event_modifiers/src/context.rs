use bevy_app::prelude::*;

pub trait EventModifierContext {
    fn register_type(app: &mut App) -> &mut App;
}
