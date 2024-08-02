# Bevy Event Modifiers

Generic event modifier pattern for [Bevy](https://bevyengine.org/).

## Usage

```rust
use bevy::prelude::*;
use bevy_event_modifiers::prelude::*;

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
}

#[derive(Event)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub critical: bool,
    pub damage: u32,
}

impl DamageEvent {
    fn init(_: &mut AttackEventContext, event: &AttackEvent) -> Self {
        DamageEvent {
            attacker: event.attacker,
            target: event.target,
            critical: false,
            damage: event.damage,
        }
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

pub fn armor_modifier(
    context: &mut AttackEventContext,
    _: &mut Metadata,
    event: &mut DamageEvent,
) {
    if let Ok(_) = context.q_invulnarable.get(event.target) {
        event.damage = 0;
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Modifier {
        priority: Priority::Armor,
        modify: armor_modifier,
    });
    ...
}

pub fn init(app: &mut App) {
    app.add_event_with_modifiers::<AttackEventContext<'_, '_>>();

    app.add_systems(Startup, setup);
}
```

Which will generate some code, most notibly:

```rust
impl<'w, 's> AttackEventContext<'w, 's> {
    pub fn system(
        mut p_events_in: EventReader<AttackEvent>,
        r_rng: ResMut<Rng>,
        q_armor: Query<&'static Armor>,
        q_critical_chance: Query<&'static CriticalChance>,
        q_invulnarable: Query<&'static Invulnerable>,
        p_modifiers: Query<&Modifier>,
        mut p_events_out: EventWriter<DamageEvent>,
    ) {
        let mut context = AttackEventContext {
            r_rng,
            q_armor,
            q_critical_chance,
            q_invulnarable,
        };
        let modifiers = p_modifiers.iter().sort::<&Modifier>().collect::<Vec<_>>();
        for event in p_events_in.read() {
            let mut metadata = Metadata::init(&mut context, event);
            let mut event_out = DamageEvent::init(&mut context, event);
            for modifier in &modifiers {
                (modifier.modify)(&mut context, &mut metadata, &mut event_out);
            }
            p_events_out.send(event_out);
        }
    }
}
```

## Testing

```rust
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
    let mut event_reader = events_out.get_reader();
    let events_out: Vec<&DamageEvent> = event_reader.read(events_out).collect();

    assert_eq!(events_out.len(), 1);
    assert_eq!(events_out[0].attacker, attacker);
    assert_eq!(events_out[0].target, target);
    assert!(!events_out[0].critical);
    assert_eq!(events_out[0].damage, 5);
}
```
