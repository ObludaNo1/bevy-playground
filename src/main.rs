mod camera;
mod characters;
mod collision;
mod config;
mod inventory;
mod map;
mod state;

use bevy::{
    prelude::*,
    window::{Window, WindowMode, WindowPlugin},
};

use bevy_procedural_tilemaps::prelude::*;

use crate::{
    camera::CameraPlugin, characters::CharactersPlugin, collision::CollisionPlugin,
    inventory::InventoryPlugin, map::generate::setup_generator, state::StatePlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
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
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .add_plugins(StatePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(CharactersPlugin)
        .add_plugins(InventoryPlugin)
        .add_systems(Startup, setup_generator)
        .run();
}
