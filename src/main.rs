// Based on https://lucasschuermann.com/writing/particle-based-fluid-simulation
// https://lucasschuermann.com/writing/implementing-sph-in-2d

pub mod particles;
pub mod math;
pub mod simulator;
pub mod renderer;

use renderer::*;
use simulator::*;
use particles::*;

fn main() {
    let mut particles = Particles::new(1024);
    let mut renderer = Renderer::new(particles.count());
    let mut simulator = Simulator::start();
    while renderer.up() {
        simulator.tick(&mut particles);
        renderer.render(&particles);
    }
}
