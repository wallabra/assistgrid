use bevy::prelude::*;

#[derive(Component)]
pub struct Physiology {
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
