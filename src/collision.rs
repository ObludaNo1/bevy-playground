mod map;
mod systems;
mod tile_type;

#[cfg(debug_assertions)]
mod debug;

use bevy::prelude::*;
#[cfg(debug_assertions)]
pub use debug::DebugCollisionEnabled;
// Re-export commonly used types
pub use map::CollisionMap;
pub use systems::CollisionMapBuilt;
pub use tile_type::{TileMarker, TileType};

use crate::state::GameState;

/// Plugin for collision detection functionality
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionMapBuilt>().add_systems(
            Update,
            systems::build_collision_map
                .run_if(resource_equals(CollisionMapBuilt(false)))
                .run_if(in_state(GameState::Playing)),
        );

        // Debug systems - only in debug builds
        #[cfg(debug_assertions)]
        {
            app.init_resource::<DebugCollisionEnabled>().add_systems(
                Update,
                (
                    debug::toggle_debug_collision,
                    debug::debug_draw_collision,
                    debug::debug_player_position,
                )
                    .run_if(in_state(GameState::Playing)),
            );
        }
    }
}
