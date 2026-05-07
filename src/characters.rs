pub mod animation;
pub mod collider;
pub mod config;
pub mod facing;
pub mod input;
pub mod physics;
pub mod rendering;
pub mod spawn;
pub mod state;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use config::CharactersList;

use crate::{characters::spawn::PlayerSpawned, collision::CollisionMapBuilt, state::GameState};

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            .init_resource::<spawn::CurrentCharacterIndex>()
            .init_resource::<PlayerSpawned>()
            .add_systems(Startup, spawn::load_character_assets)
            .add_systems(
                Update,
                spawn::spawn_player_at_valid_position
                    .run_if(resource_equals(CollisionMapBuilt(true)))
                    .run_if(resource_equals(PlayerSpawned(false)))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    // 1. Input determines state + velocity + facing
                    input::handle_player_input,
                    spawn::switch_character,
                    input::update_jump_state,
                    // 2. State changes trigger animation updates
                    animation::on_state_change_update_animation,
                    // 3. Collision validation adjusts velocity
                    collider::validate_movement,
                    collider::resolve_entity_collisions,
                    // 4. Physics applies velocity to transform
                    physics::apply_velocity,
                    rendering::update_character_depth,
                    // 5. Animation ticks frames
                    animation::animations_playback,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
