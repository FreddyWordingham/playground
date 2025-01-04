use bevy::prelude::*;

#[derive(Event)]
pub struct HealEvent {
    pub target: Entity,
    pub heal: u32,
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: u32,
}

impl DamageEvent {
    pub fn new(target: Entity, damage: u32) -> Self {
        debug_assert!(damage > 0);
        Self { target, damage }
    }
}

#[derive(Component)]
pub struct Health {
    pub value: u32,
    pub max_value: u32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            value: 1000,
            max_value: 1000,
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealEvent>()
            .add_event::<DamageEvent>()
            .add_systems(
                Update,
                (process_healing_events, process_damage_events).chain(),
            );
    }
}

pub fn process_healing_events(
    mut heal_events: EventReader<HealEvent>,
    mut query: Query<&mut Health>,
) {
    for event in heal_events.read() {
        if let Ok(mut health) = query.get_mut(event.target) {
            health.value = (health.value + event.heal).min(health.max_value);
        }
    }
}

pub fn process_damage_events(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
    mut query: Query<&mut Health>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = query.get_mut(event.target) {
            health.value = health.value.saturating_sub(event.damage);

            if health.value == 0 {
                commands.entity(event.target).despawn_recursive();
            }
        }
    }
}
