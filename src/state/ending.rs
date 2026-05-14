use bevy::prelude::*;

use crate::{
    characters::{input::Player, spawn::PlayerSpawned},
    collision::{CollisionMap, CollisionMapBuilt, TileMarker},
    combat::{
        healthbar::HealthBarOwner,
        systems::{Projectile, ProjectileEffect},
    },
    enemy::{components::Enemy, spawn::EnemiesSpawned},
    inventory::Inventory,
    map::generate::MapReady,
    particles::components::{Particle, ParticleEmitter},
    state::GameState,
};

pub fn handle_restart_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        info!("Restarting game...");
        // Restart must re-run loading/setup systems (map generation, flags/resources).
        next_state.set(GameState::Loading);
    }
}

/// Despawns all gameplay entities and resets spawn flags so they re-trigger.
pub fn cleanup_game_world(
    mut commands: Commands,
    tiles: Query<Entity, With<TileMarker>>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    projectiles: Query<Entity, With<Projectile>>,
    projectile_effects: Query<Entity, With<ProjectileEffect>>,
    emitters: Query<Entity, With<ParticleEmitter>>,
    particles: Query<Entity, With<Particle>>,
    healthbars: Query<Entity, With<HealthBarOwner>>,
    mut player_spawned: ResMut<PlayerSpawned>,
    mut enemies_spawned: ResMut<EnemiesSpawned>,
    mut collision_map_built: ResMut<CollisionMapBuilt>,
    mut inventory: ResMut<Inventory>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn();
    }
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemies.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectiles.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectile_effects.iter() {
        commands.entity(entity).despawn();
    }
    for entity in emitters.iter() {
        commands.entity(entity).despawn();
    }
    for entity in particles.iter() {
        commands.entity(entity).despawn();
    }
    for entity in healthbars.iter() {
        commands.entity(entity).despawn();
    }

    player_spawned.0 = false;
    enemies_spawned.0 = false;

    collision_map_built.0 = false;
    commands.remove_resource::<CollisionMap>();
    inventory.set_items(Default::default());
    commands.remove_resource::<MapReady>();
}
