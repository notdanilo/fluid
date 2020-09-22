use crate::math::*;
use crate::particles::Particle;
use crate::simulator::constants::MASS;

pub trait Force {
    fn force(&self, particle: &Particle) -> Vector;
}

pub struct Gravity {
    intensity: f32
}

impl Gravity {
    pub fn new(intensity: f32) -> Self {
        Self { intensity }
    }
}

impl Force for Gravity {
    fn force(&self, particle: &Particle) -> Vector {
        Vector::new(0.0, self.intensity * MASS, 0.0)
    }
}