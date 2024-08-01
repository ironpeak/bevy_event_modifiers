use crate::prelude::*;

#[derive(Event)]
pub struct HitEventIn {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: u32,
}

#[derive(EventModifierContext)]
pub struct HitEventModifierContext<'w, 's> {
    pub r_rng: ResMut<'w, Rng>,
    pub q_armor: Query<'w, 's, &'static Armor>,
    pub q_critical_chance: Query<'w, 's, &'static CriticalChance>,
    pub q_invulnarable: Query<'w, 's, &'static Invulnerable>,
}

#[derive(Event)]
pub struct HitEventOut {
    pub attacker: Entity,
    pub target: Entity,
    pub critical: bool,
    pub damage: u32,
}

impl HitEventOut {
    fn init(_: &mut HitEventModifierContext, event: &HitEventIn) -> Self {
        HitEventOut {
            attacker: event.attacker,
            target: event.target,
            critical: false,
            damage: event.damage,
        }
    }
}

pub struct HitEventModifierMetadata {}

impl HitEventModifierMetadata {
    fn init(_: &mut HitEventModifierContext, _: &HitEventIn) -> Self {
        HitEventModifierMetadata {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HitEventModifierPriority {
    Armor,
    Critical,
    Invulnerable,
}

pub fn armor_modifier(
    context: &mut HitEventModifierContext,
    _: &mut HitEventModifierMetadata,
    event: &mut HitEventOut,
) {
    if let Ok(armor) = context.q_armor.get(event.target) {
        event.damage = event.damage.saturating_sub(armor.value);
    }
}

pub fn critical_modifier(
    context: &mut HitEventModifierContext,
    _: &mut HitEventModifierMetadata,
    event: &mut HitEventOut,
) {
    if let Ok(critical_chance) = context.q_critical_chance.get(event.attacker) {
        if context.r_rng.rng.next_u32() % 100 < critical_chance.value {
            event.critical = true;
            event.damage *= 2;
        }
    }
}

pub fn invulnerable_modifier(
    context: &mut HitEventModifierContext,
    _: &mut HitEventModifierMetadata,
    event: &mut HitEventOut,
) {
    if let Ok(_) = context.q_invulnarable.get(event.target) {
        event.damage = 0;
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Rng {
        rng: StdRng::seed_from_u64(rand::thread_rng().next_u64()),
    });
    commands.spawn(HitEventModifier {
        priority: HitEventModifierPriority::Armor,
        modify: armor_modifier,
    });
    commands.spawn(HitEventModifier {
        priority: HitEventModifierPriority::Critical,
        modify: critical_modifier,
    });
    commands.spawn(HitEventModifier {
        priority: HitEventModifierPriority::Invulnerable,
        modify: invulnerable_modifier,
    });
}

pub fn init(app: &mut App) {
    app.add_event_with_modifiers::<HitEventModifierContext<'_, '_>>();

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

        world.spawn(HitEventModifier {
            priority: HitEventModifierPriority::Armor,
            modify: armor_modifier,
        });

        world.insert_resource(Events::<HitEventIn>::default());
        world.insert_resource(Events::<HitEventOut>::default());

        let mut events_in = world.resource_mut::<Events<HitEventIn>>();
        events_in.send(HitEventIn {
            attacker,
            target,
            damage: 10,
        });

        let action = world.register_system(HitEventModifierContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<HitEventOut>>();
        let mut event_reader = events_out.get_reader();
        let events_out: Vec<&HitEventOut> = event_reader.read(events_out).collect();

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

        world.spawn(HitEventModifier {
            priority: HitEventModifierPriority::Critical,
            modify: critical_modifier,
        });

        world.insert_resource(Events::<HitEventIn>::default());
        world.insert_resource(Events::<HitEventOut>::default());

        let mut events_in = world.resource_mut::<Events<HitEventIn>>();
        events_in.send(HitEventIn {
            attacker,
            target,
            damage: 10,
        });

        let action = world.register_system(HitEventModifierContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<HitEventOut>>();
        let mut event_reader = events_out.get_reader();
        let events_out: Vec<&HitEventOut> = event_reader.read(events_out).collect();

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

        world.spawn(HitEventModifier {
            priority: HitEventModifierPriority::Invulnerable,
            modify: invulnerable_modifier,
        });

        world.insert_resource(Events::<HitEventIn>::default());
        world.insert_resource(Events::<HitEventOut>::default());

        let mut events_in = world.resource_mut::<Events<HitEventIn>>();
        events_in.send(HitEventIn {
            attacker,
            target,
            damage: 10,
        });

        let action = world.register_system(HitEventModifierContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<HitEventOut>>();
        let mut event_reader = events_out.get_reader();
        let events_out: Vec<&HitEventOut> = event_reader.read(events_out).collect();

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

        world.spawn(HitEventModifier {
            priority: HitEventModifierPriority::Armor,
            modify: armor_modifier,
        });
        world.spawn(HitEventModifier {
            priority: HitEventModifierPriority::Critical,
            modify: critical_modifier,
        });
        world.spawn(HitEventModifier {
            priority: HitEventModifierPriority::Invulnerable,
            modify: invulnerable_modifier,
        });

        world.insert_resource(Events::<HitEventIn>::default());
        world.insert_resource(Events::<HitEventOut>::default());

        let mut events_in = world.resource_mut::<Events<HitEventIn>>();
        events_in.send(HitEventIn {
            attacker,
            target,
            damage: 12,
        });

        let action = world.register_system(HitEventModifierContext::system);
        world.run_system(action).unwrap();

        let events_out = world.resource::<Events<HitEventOut>>();
        let mut event_reader = events_out.get_reader();
        let events_out: Vec<&HitEventOut> = event_reader.read(events_out).collect();

        assert_eq!(events_out.len(), 1);
        assert_eq!(events_out[0].attacker, attacker);
        assert_eq!(events_out[0].target, target);
        assert!(events_out[0].critical);
        assert_eq!(events_out[0].damage, 14);
    }
}
