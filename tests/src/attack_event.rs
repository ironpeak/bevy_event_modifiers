use crate::prelude::*;

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: u32,
}

#[derive(EventModifierContext)]
#[modifier(
    input = AttackEvent,
    metadata = Metadata,
    priority = Priority,
    component = Modifier,
    output = DamageEvent
)]
pub(crate) struct AttackEventContext<'w, 's> {
    pub r_rng: ResMut<'w, Rng>,
    pub q_armor: Query<'w, 's, &'static Armor>,
    pub q_critical_chance: Query<'w, 's, &'static CriticalChance>,
    pub q_invulnarable: Query<'w, 's, &'static Invulnerable>,
    pub e_invulnerability: EventWriter<'w, InvulnerabilityEvent>,
}

#[derive(Event)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub critical: bool,
    pub damage: u32,
}

impl DamageEvent {
    fn init(_: &mut AttackEventContext, event: &AttackEvent) -> Option<Self> {
        Some(DamageEvent {
            attacker: event.attacker,
            target: event.target,
            critical: false,
            damage: event.damage,
        })
    }
}

pub(crate) struct Metadata {}

impl Metadata {
    fn init(_: &mut AttackEventContext, _: &AttackEvent) -> Self {
        Metadata {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Priority {
    Armor,
    Critical,
    Invulnerable,
}

pub fn armor_modifier(context: &mut AttackEventContext, _: &mut Metadata, event: &mut DamageEvent) {
    if let Ok(armor) = context.q_armor.get(event.target) {
        event.damage = event.damage.saturating_sub(armor.value);
    }
}

pub fn critical_modifier(
    context: &mut AttackEventContext,
    _: &mut Metadata,
    event: &mut DamageEvent,
) {
    if let Ok(critical_chance) = context.q_critical_chance.get(event.attacker) {
        if context.r_rng.rng.next_u32() % 100 < critical_chance.value {
            event.critical = true;
            event.damage *= 2;
        }
    }
}

pub fn invulnerable_modifier(
    context: &mut AttackEventContext,
    _: &mut Metadata,
    event: &mut DamageEvent,
) {
    if let Ok(_) = context.q_invulnarable.get(event.target) {
        context.e_invulnerability.send(InvulnerabilityEvent {});
        event.damage = 0;
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Rng {
        rng: StdRng::seed_from_u64(rand::thread_rng().next_u64()),
    });
    commands.spawn(Modifier {
        priority: Priority::Armor,
        modify: armor_modifier,
    });
    commands.spawn(Modifier {
        priority: Priority::Critical,
        modify: critical_modifier,
    });
    commands.spawn(Modifier {
        priority: Priority::Invulnerable,
        modify: invulnerable_modifier,
    });
}

pub fn init(app: &mut App) {
    app.add_event_with_modifiers::<AttackEventContext<'_, '_>>();

    app.add_systems(Startup, setup);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armor() {
        let mut world = World::default();

        let attacker = world
            .spawn((
                Armor { value: 3 },
                CriticalChance { value: 30 },
                Invulnerable {},
            ))
            .id();
        let target = world
            .spawn((
                Armor { value: 5 },
                CriticalChance { value: 0 },
                Invulnerable {},
            ))
            .id();

        world.insert_resource(Rng {
            rng: StdRng::seed_from_u64(0),
        });

        world.spawn(Modifier {
            priority: Priority::Armor,
            modify: armor_modifier,
        });

        world.insert_resource(Events::<InvulnerabilityEvent>::default());
        world.insert_resource(Events::<AttackEvent>::default());
        world.insert_resource(Events::<DamageEvent>::default());

        let mut events_in = world.resource_mut::<Events<AttackEvent>>();
        events_in.send(AttackEvent {
            attacker,
            target,
            damage: 10,
        });

        let action = world.register_system(AttackEventContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<DamageEvent>>();
        let mut event_reader = events_out.get_cursor();
        let events_out: Vec<&DamageEvent> = event_reader.read(events_out).collect();

        assert_eq!(events_out.len(), 1);
        assert_eq!(events_out[0].attacker, attacker);
        assert_eq!(events_out[0].target, target);
        assert!(!events_out[0].critical);
        assert_eq!(events_out[0].damage, 5);
    }

    #[test]
    fn test_critical() {
        let mut world = World::default();

        let attacker = world
            .spawn((
                Armor { value: 3 },
                CriticalChance { value: 30 },
                Invulnerable {},
            ))
            .id();
        let target = world
            .spawn((
                Armor { value: 5 },
                CriticalChance { value: 0 },
                Invulnerable {},
            ))
            .id();

        world.insert_resource(Rng {
            rng: StdRng::seed_from_u64(0),
        });

        world.spawn(Modifier {
            priority: Priority::Critical,
            modify: critical_modifier,
        });

        world.insert_resource(Events::<InvulnerabilityEvent>::default());
        world.insert_resource(Events::<AttackEvent>::default());
        world.insert_resource(Events::<DamageEvent>::default());

        let mut events_in = world.resource_mut::<Events<AttackEvent>>();
        events_in.send(AttackEvent {
            attacker,
            target,
            damage: 10,
        });

        let action = world.register_system(AttackEventContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<DamageEvent>>();
        let mut event_reader = events_out.get_cursor();
        let events_out: Vec<&DamageEvent> = event_reader.read(events_out).collect();

        assert_eq!(events_out.len(), 1);
        assert_eq!(events_out[0].attacker, attacker);
        assert_eq!(events_out[0].target, target);
        assert!(events_out[0].critical);
        assert_eq!(events_out[0].damage, 20);
    }

    #[test]
    fn test_invulnerable() {
        let mut world = World::default();

        let attacker = world
            .spawn((
                Armor { value: 3 },
                CriticalChance { value: 30 },
                Invulnerable {},
            ))
            .id();
        let target = world
            .spawn((
                Armor { value: 5 },
                CriticalChance { value: 0 },
                Invulnerable {},
            ))
            .id();

        world.insert_resource(Rng {
            rng: StdRng::seed_from_u64(0),
        });

        world.spawn(Modifier {
            priority: Priority::Invulnerable,
            modify: invulnerable_modifier,
        });

        world.insert_resource(Events::<InvulnerabilityEvent>::default());
        world.insert_resource(Events::<AttackEvent>::default());
        world.insert_resource(Events::<DamageEvent>::default());

        let mut events_in = world.resource_mut::<Events<AttackEvent>>();
        events_in.send(AttackEvent {
            attacker,
            target,
            damage: 10,
        });

        let action = world.register_system(AttackEventContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<DamageEvent>>();
        let mut event_reader = events_out.get_cursor();
        let events_out: Vec<&DamageEvent> = event_reader.read(events_out).collect();

        assert_eq!(events_out.len(), 1);
        assert_eq!(events_out[0].attacker, attacker);
        assert_eq!(events_out[0].target, target);
        assert!(!events_out[0].critical);
        assert_eq!(events_out[0].damage, 0);
    }

    #[test]
    fn test_all() {
        let mut world = World::default();

        let attacker = world
            .spawn((
                Armor { value: 3 },
                CriticalChance { value: 30 },
                Invulnerable {},
            ))
            .id();
        let target = world
            .spawn((Armor { value: 5 }, CriticalChance { value: 0 }))
            .id();

        world.insert_resource(Rng {
            rng: StdRng::seed_from_u64(0),
        });

        world.spawn(Modifier {
            priority: Priority::Armor,
            modify: armor_modifier,
        });
        world.spawn(Modifier {
            priority: Priority::Critical,
            modify: critical_modifier,
        });
        world.spawn(Modifier {
            priority: Priority::Invulnerable,
            modify: invulnerable_modifier,
        });

        world.insert_resource(Events::<InvulnerabilityEvent>::default());
        world.insert_resource(Events::<AttackEvent>::default());
        world.insert_resource(Events::<DamageEvent>::default());

        let mut events_in = world.resource_mut::<Events<AttackEvent>>();
        events_in.send(AttackEvent {
            attacker,
            target,
            damage: 12,
        });

        let action = world.register_system(AttackEventContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<DamageEvent>>();
        let mut event_reader = events_out.get_cursor();
        let events_out: Vec<&DamageEvent> = event_reader.read(events_out).collect();

        assert_eq!(events_out.len(), 1);
        assert_eq!(events_out[0].attacker, attacker);
        assert_eq!(events_out[0].target, target);
        assert!(events_out[0].critical);
        assert_eq!(events_out[0].damage, 14);
    }
}
