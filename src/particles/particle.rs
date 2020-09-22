use crate::math::*;

#[derive(Clone)]
pub struct Particle {
    pub id: uuid::Uuid,
    pub position: Vector,
    pub velocity: Vector,
    pub force: Vector,
    pub density: f32,
    pub pressure: f32
}

impl Particle {
    pub fn new(position: Vector, velocity: Vector) -> Self {
        let id = uuid::Uuid::new_v4();
        let density = 0.0;
        let pressure = 0.0;
        let force = zero();
        Self { id, position, velocity, density, pressure, force }
    }
}
