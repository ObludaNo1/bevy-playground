use bevy::prelude::*;

#[derive(Component)]
pub struct VictoryScreen;

pub fn spawn_victory_screen(mut commands: Commands) {
    commands
        .spawn((
            VictoryScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("WELL DONE YOUNG PADAWAN!\n\nPress R to restart"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });

    info!("Victory screen spawned");
}

pub fn despawn_victory_screen(mut commands: Commands, query: Query<Entity, With<VictoryScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    info!("Victory screen despawned");
}
