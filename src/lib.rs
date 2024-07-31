use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, query::{QuerySortedIter, WorldQuery}};

pub mod prelude;

pub struct EventModifiersPlugin {}

impl Plugin for EventModifiersPlugin {
    fn build(&self, _: &mut App) {}
}

// #[derive(PartialEq, Eq, PartialOrd, Ord)]
// #[repr(u8)]
// pub enum CombatDamageModifierPriority {
//     _TODO,
// }

// pub struct CombatDamageModifierContext {
//     pub _attacker: Entity,
//     pub _target: Entity,
//     pub knockback: Option<CombatDamageModifierContextKnockback>,
//     pub damage: u32,
// }

// #[derive(Debug, Clone, PartialEq, Event)]
// pub struct CombatDamageModifierContextKnockback {
//     pub direction: Vec2,
//     pub force: f32,
//     pub time: f32,
// }

// #[derive(Component)]
// pub struct CombatDamageModifierComponent {
//     pub priority: CombatDamageModifierPriority,
//     pub function: fn(&mut CombatDamageModifierContext),
// }
pub trait EventModifiersAppExt {
    fn add_event_with_modifiers<E: Event>(
        &mut self,
    ) -> &mut Self;
}

impl EventModifiersAppExt for App {
    fn add_event_with_modifiers<E: Event>(
        &mut self,
    ) -> &mut Self {
        todo!()
    }
}

#[derive(Component)]
pub struct CombatDamageModifierComponent<Priority> {
    pub priority: Priority,
    // pub function: fn(&mut CombatDamageModifierContext),
}

impl<Priority> Ord for CombatDamageModifierComponent<Priority> where Priority: Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<Priority> PartialOrd for CombatDamageModifierComponent<Priority> where Priority: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<Priority> Eq for CombatDamageModifierComponent<Priority> where Priority: Eq {

}

impl<Priority> PartialEq for CombatDamageModifierComponent<Priority> where Priority: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

fn system<ModifierPriority>(modifiers: Query<&CombatDamageModifierComponent<ModifierPriority>>) 
where ModifierPriority: Eq + PartialEq + Ord + PartialOrd + Send + Sync + 'static {
    for modifier in modifiers
        .iter()
        .sort::<&CombatDamageModifierComponent<ModifierPriority>>() {}
    // for event in defense_events.read() {
    //     let Ok((mut damage, health)) = targets.get_mut(event.target) else {
    //         continue;
    //     };
    //     let mut context = CombatDamageModifierContext {
    //         _attacker: event.attacker,
    //         _target: event.target,
    //         knockback: event.knockback.as_ref().map(|knockback| {
    //             CombatDamageModifierContextKnockback {
    //                 direction: knockback.direction,
    //                 force: knockback.force,
    //                 time: knockback.time,
    //             }
    //         }),
    //         damage: event.damage,
    //     };
    //     for damage in &damages {
    //         (damage.function)(&mut context);
    //     }
    //     damage.base += context.damage;
    //     let lethal = damage.base >= health.value.current();
    //     damage_events.send(CombatDamageEvent {
    //         attacker: event.attacker,
    //         target: event.target,
    //         knockback: context
    //             .knockback
    //             .map(|knockback| CombatDamageEventKnockback {
    //                 direction: knockback.direction,
    //                 force: knockback.force,
    //                 time: knockback.time,
    //             }),
    //         position: event.position,
    //         damage: context.damage,
    //         lethal,
    //     });
    // }
}
