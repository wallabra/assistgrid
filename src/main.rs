use assistgrid::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                initial_item_types,
                initial_friction::<{ (0.7 * 128.0) as i64 }>,
            )
                .before(initial_people),
        )
        .add_systems(Startup, initial_people)
        .add_systems(Update, (hello_world_system, update_movement))
        .run();
}

fn hello_world_system() {
    println!("hello world");
}
