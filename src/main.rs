use bevy::prelude::*;
use std::collections::HashMap;
use vector2d::Vector2D as Vec2;

pub mod item;

#[derive(Component)]
struct Movement {
    pub pos: Vec2<i64>,
    pub vel: Vec2<i64>,
}

impl Movement {
    fn update_pos(&mut self, time: f64) {
        self.pos += (self.vel.as_f64s() * time).as_i64s();
    }

    fn update_friction(&mut self, time: f64, factor: f64) {
        self.vel -= (self.vel.as_f64s().normalise()
            * (factor * time).min(self.vel.as_f64s().length()))
        .as_i64s();
    }

    pub fn update(&mut self, time: f64, friction: f64) {
        self.update_pos(time);
        self.update_friction(time, friction);
    }

    pub fn apply_force(&mut self, force: Vec2<i64>) {
        self.vel += force;
    }
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Wallet(i32);

#[derive(Component)]
struct Physiology {
    pub hunger: f64,
    pub boredom: f64,
    pub energy: f64,
}

impl Default for Physiology {
    fn default() -> Self {
        Physiology {
            hunger: 0.5,
            boredom: 0.7,
            energy: 0.2,
        }
    }
}

impl Physiology {
    pub fn eat(&mut self, amount: f64) {
        self.hunger += amount;

        if self.hunger >= 1.0 {
            self.hunger = 1.0;
        }
    }

    pub fn entertain(&mut self, amount: f64) {
        self.boredom -= amount;

        if self.boredom < 0.0 {
            self.boredom = 0.0;
        }
    }

    pub fn update(&mut self, time: f64) {
        self.hunger -= time * 0.01 * time;
        self.boredom += time * 0.01 * time;
        self.energy -= time * 0.001 * time;

        if self.hunger <= 0.0 {
            self.hunger = 0.0;
        }

        if self.boredom > 1.0 {
            self.boredom = 1.0;
        }

        if self.energy < 0.0 {
            self.energy = 0.0;
        }

        if self.energy < 1.0 {
            let delta = (1.0 - self.energy).min(0.02 * time);
            if self.hunger > 2.0 * delta {
                self.energy += delta;
                self.hunger -= 2.0 * delta;
            }
        }
    }
}

#[derive(Component)]
struct Inventory(HashMap<usize, item::Item>);

#[derive(Resource)]
struct WorldFriction(f64);

fn make_person(commands: &mut Commands, name: &str, init_wallet: Option<i32>) {
    commands.spawn((
        Person,
        Name(name.to_owned()),
        Wallet(init_wallet.unwrap_or(0)),
        Physiology::default(),
    ));
}

fn initial_people(mut commands: Commands) {
    make_person(&mut commands, "Albert", Some(500));
    make_person(&mut commands, "Gemin", Some(500));
    make_person(&mut commands, "Kraus", Some(900));
}

fn update_movement(mut query: Query<&mut Movement>, time: Res<Time>, friction: Res<WorldFriction>) {
    for mut movement in &mut query {
        movement.update(time.delta().as_secs_f64(), friction.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, initial_people)
        .add_systems(Update, (hello_world_system, update_movement))
        .run();
}

fn hello_world_system() {
    println!("hello world");
}
