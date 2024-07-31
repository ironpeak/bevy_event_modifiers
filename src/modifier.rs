use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct ModifierComponent<Priority, Context> {
    pub priority: Priority,
    pub modify: fn(&mut Context),
}

impl<Priority, Context> Ord for ModifierComponent<Priority, Context>
where
    Priority: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<Priority, Context> PartialOrd for ModifierComponent<Priority, Context>
where
    Priority: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<Priority, Context> Eq for ModifierComponent<Priority, Context> where Priority: Eq {}

impl<Priority, Context> PartialEq for ModifierComponent<Priority, Context>
where
    Priority: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
