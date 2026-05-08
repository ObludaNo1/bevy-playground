// src/combat/observers.rs
use bevy::prelude::*;

use super::{
    events::{EntityDeath, ProjectileHit},
    health::Health,
};
use crate::{characters::input::Player, enemy::components::Enemy, state::GameState};

/// Observer that handles projectile hits by applying damage to the target.
pub fn on_projectile_hit(
    hit: On<ProjectileHit>,
    mut healths: Query<&mut Health>,
    mut commands: Commands,
) {
    let Ok(mut health) = healths.get_mut(hit.target) else {
        return;
    };

    health.take_damage(&mut commands, hit.target, hit.damage);

    info!(
        "{:?} hit for {} damage! HP: {:.0}/{:.0}",
        hit.power_type, hit.damage, health.current, health.max
    );
}

/// Observer that handles entity death by despawning the entity.
pub fn on_entity_death(
    death: On<EntityDeath>,
    mut commands: Commands,
    players: Query<(), With<Player>>,
    enemies: Query<(), With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let entity = death.entity;
    let is_player = players.get(entity).is_ok();
    let is_enemy = enemies.get(entity).is_ok();

    info!("Entity {:?} defeated!", death.entity);
    commands.entity(death.entity).despawn();

    if is_player {
        info!("Player defeated! Game Over.");
        next_state.set(GameState::GameOver);
    }

    // Why `enemies.iter().count() <= 1` is safe for simultaneous deaths
    //
    // `commands.entity(...).despawn()` and `commands.trigger(EntityDeath { ... })` are both
    // deferred — they are queued and flushed one at a time. Bevy flushes each triggered observer's
    // own commands before processing the next queued trigger, so two deaths triggered on the same
    // frame are handled sequentially: by the time the second `EntityDeath` observer runs, the first
    // enemy is already despawned. The count is therefore always accurate.
    if is_enemy && enemies.iter().count() <= 1 {
        // This is the last enemy (despawn is deferred, so it still appears in the query)
        info!("All enemies defeated! Victory!");
        next_state.set(GameState::Victory);
    }
}
