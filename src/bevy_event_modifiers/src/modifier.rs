use bevy_app::prelude::*;

pub trait EventModifier {
    fn register_type(app: &mut App) -> &mut App;
}
