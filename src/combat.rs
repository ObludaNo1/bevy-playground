pub mod events;
pub mod health;
pub mod healthbar;
pub mod observers;
pub mod player_combat;
pub mod power_type;
pub mod systems;

pub use player_combat::PlayerCombat;
pub use power_type::PowerType;
pub use systems::{debug_switch_power, handle_power_input};

use bevy::prelude::*;

use crate::{
    combat::{
        healthbar::{spawn_healthbars, update_healthbars},
        observers::{on_entity_death, on_projectile_hit},
        systems::{check_projectile_hits, move_projectiles},
    },
    state::GameState,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register observers for combat events
            .add_observer(on_projectile_hit)
            .add_observer(on_entity_death)
            // Update the following systems
            .add_systems(
                Update,
                (
                    handle_power_input,
                    debug_switch_power,
                    move_projectiles,
                    check_projectile_hits,
                    spawn_healthbars,
                    update_healthbars,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
