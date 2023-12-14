use bevy::prelude::*;
use vector2d::Vector2D as Vec2;

pub mod item;
pub mod living;
pub mod physics;

use item::ItemTypes;
use living::Physiology;
use physics::Movement;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Wallet(i32);

#[derive(Resource)]
pub struct WorldFriction(f64);

pub fn make_person(
    commands: &mut Commands,
    itemtypes: &ItemTypes,
    name: &str,
    init_wallet: Option<i32>,
) {
    commands.spawn((
        Person,
        Name(name.to_owned()),
        Wallet(init_wallet.unwrap_or(0)),
        Physiology::default(),
        Movement::default().apply_force_owned(Vec2::new(5, 0)),
        itemtypes.new_empty_inventory(),
    ));
}

pub fn initial_people(mut commands: Commands, itemtypes: Res<ItemTypes>) {
    make_person(&mut commands, &itemtypes, "Albert", Some(500));
    make_person(&mut commands, &itemtypes, "Gemin", Some(500));
    make_person(&mut commands, &itemtypes, "Kraus", Some(900));
}

pub fn initial_friction<const FRICTION: i64>(mut commands: Commands) {
    commands.insert_resource(WorldFriction((FRICTION as f64) / 128.0));
}

pub fn initial_item_types(mut commands: Commands) {
    commands.insert_resource(ItemTypes::default());
}

pub fn update_movement(
    mut query: Query<&mut Movement>,
    time: Res<Time>,
    friction: Res<WorldFriction>,
) {
    for mut movement in &mut query {
        movement.update(time.delta().as_secs_f64(), friction.0);
    }
}
