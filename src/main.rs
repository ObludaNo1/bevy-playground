mod camera;
mod characters;
mod collision;
mod combat;
mod config;
mod enemy;
mod inventory;
mod map;
mod particles;
mod state;

use bevy::{
    prelude::*,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin},
};

use crate::{
    camera::CameraPlugin,
    characters::CharactersPlugin,
    collision::CollisionPlugin,
    combat::CombatPlugin,
    enemy::EnemyPlugin,
    inventory::InventoryPlugin,
    map::generate::{poll_map_generation, setup_generator},
    particles::ParticlesPlugin,
    state::{GameState, StatePlugin},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Game".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(StatePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(CharactersPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(InventoryPlugin)
        .add_systems(Startup, setup_generator)
        .add_plugins(CombatPlugin)
        .add_plugins(ParticlesPlugin)
        .add_systems(
            Update,
            poll_map_generation.run_if(in_state(GameState::Loading)),
        )
        .run();
}
