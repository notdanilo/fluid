use crate::math::*;

mod particle;
mod cell;

pub use particle::Particle;
pub use cell::Cell;
use crate::simulator::constants::*;

#[derive(Clone)]
pub struct Particles {
    pub particles: Vec<Particle>
}

impl Particles {
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::new();

        let size = (count as f32).sqrt();

        const VIEW_WIDTH: f32 = 1920.0;
        const VIEW_HEIGHT: f32 = 1080.0;
        let mut x = EPS;
        let mut y = EPS;
        for _ in 0..count {
            let position = Vector::new(x + random(), y, 0.0);
            let velocity = zero();
            particles.push(Particle::new(position, velocity));
            y += H;
            if y >= VIEW_HEIGHT/2.0 {
                y = EPS;
                x += H;
            }
        }

        Self { particles }
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }
}