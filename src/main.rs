use assistgrid::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_startup_systems(initial_item_types)
        .init_resource::<assistgrid::item::ItemTypes>()
        .add_systems(Startup, initial_friction::<{ (0.7 * 128.0) as i64 }>)
        .add_systems(Startup, initial_people.after(initial_item_types))
        .add_systems(Update, (hello_world_system, update_movement))
        .run();
}

fn hello_world_system() {
    println!("hello world");
}
