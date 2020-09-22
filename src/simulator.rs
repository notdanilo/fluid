use std::time::Instant;
use crate::math::*;
use crate::particles::Particles;
use crate::simulator::force::Gravity;

pub mod constants;
use constants::*;

pub mod force;

pub struct Simulator {
    then: Instant
}

impl Simulator {
    pub fn start() -> Self {
        let then = Instant::now();
        Self { then }
    }

    pub fn tick(&mut self, particles: &mut Particles) {
        let now = Instant::now();
        let delta_time = (now - self.then).as_secs_f32();
        self.then = now;

        self.compute_density_pressure(particles);
        self.compute_forces(particles);
        self.integrate(particles, 0.001);
        //self.simulate(particles, 0.01);
    }

    fn compute_density_pressure(&mut self, particles: &mut Particles) {
        let cloned = particles.clone();
        for pi in &mut particles.particles {
            pi.density = 0.0;
            for pj in &cloned.particles {
                let rij = pj.position - pi.position;
                let r2 = rij.norm_squared();

                if r2 < HSQ {
                    pi.density += MASS * POLY6 * (HSQ - r2).powf(3.0);
                }
            }
            pi.pressure = GAS_CONSTANT * (pi.density - REST_DENSITY);
        }
    }

    fn compute_forces(&mut self, particles: &mut Particles) {
        let cloned = particles.clone();
        for pi in &mut particles.particles {
            let mut pressure  = Vector::new(0.0, 0.0, 0.0);
            let mut viscosity = Vector::new(0.0, 0.0, 0.0);
            let id = pi.id;
            for pj in cloned.particles.iter().filter(|pj| pj.id != id) {
                let rij = pj.position - pi.position;
                let r = rij.norm();
                if r < H {
                    pressure  += -rij.normalize() * MASS * (pi.pressure + pj.pressure) / (2.0 * pj.density) * SPIKY_GRAD * (H - r).powf(2.0);
                    viscosity += VISCOSITY * MASS * (pj.velocity - pi.velocity) / pj.density * VISCOSITY_LAP * (H - r);
                }
                let gravity = Vector::new(0.0, G, 0.0) * pi.density;
                pi.force = pressure + viscosity + gravity;
            }
        }
    }

    fn integrate(&self, particles: &mut Particles, delta_time: f32) {
        for p in &mut particles.particles {
            // forward Euler integration
            p.velocity += delta_time * p.force / p.density;
            p.position += delta_time * p.velocity;

            // enforce boundary conditions
            if p.position.x - EPS < 0.0 {
                p.velocity.x *= BOUND_DAMPING;
                p.position.x = EPS;
            } else if p.position.x + EPS > 1920.0 {
                p.velocity.x *= BOUND_DAMPING;
                p.position.x = 1920.0 - EPS;
            }
            if p.position.y - EPS < 0.0 {
                p.velocity.y *= BOUND_DAMPING;
                p.position.y = EPS;
            } else if p.position.y + EPS > 1080.0 {
                p.velocity.y *= BOUND_DAMPING;
                p.position.x = 1080.0 - EPS;
            }
        }
    }

    fn simulate(&self, particles: &mut Particles, delta_time: f32) {
        for particle in &mut particles.particles {
            let acceleration = particle.force / MASS;
            particle.velocity += acceleration * delta_time;
            particle.position += particle.velocity * delta_time;
            particle.position.y = particle.position.y.max(-1.0);
        }
    }
}