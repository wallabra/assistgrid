use bevy::prelude::*;
use vector2d::Vector2D as Vec2;

#[derive(Component)]
pub struct Movement {
    pub pos: Vec2<i64>,
    pub vel: Vec2<i64>,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            pos: Vec2::new(0, 0),
            vel: Vec2::new(0, 0),
        }
    }
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

    pub fn apply_force_owned(mut self, force: Vec2<i64>) -> Self {
        self.vel += force;
        self
    }

    pub fn grid_pos(&self) -> Vec2<i64> {
        self.pos / (2i64.pow(12))
    }
}
