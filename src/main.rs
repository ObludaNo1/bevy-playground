// use bevy::prelude::*;

// #[derive(Component)]
// struct Name(String);

fn hello_world() {
    println!("Hello, world!");
}

// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name("Elaina Proctor".to_string())));
//     commands.spawn((Person, Name("Renzo Hume".to_string())));
//     commands.spawn((Person, Name("Zayna Nieves".to_string())));
// }

// fn main() {
//     fn main() {
//         App::new()
//             .add_systems(Startup, add_people)
//             .add_systems(Update, hello_world)
//             .run();
//     }
// }
fn main() {
    hello_world();
}
