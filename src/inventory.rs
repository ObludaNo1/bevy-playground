pub mod inventory;
pub mod systems;

use bevy::prelude::*;

use crate::state::GameState;

pub use inventory::{Inventory, ItemKind, Pickable};
use systems::handle_pickups;

/// Plugin for inventory and pickup functionality.
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(Update, handle_pickups.run_if(in_state(GameState::Playing)));
    }
}
