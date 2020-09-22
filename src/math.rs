pub const DIMENSIONS: usize = 3;

pub type Vector = nalgebra::Vector3<f32>;

pub fn random() -> f32 {
    rand::random::<f32>() * 2.0 - 1.0
}

pub fn vrandom() -> Vector {
    Vector::new(random(), random(), random())
}

pub use nalgebra::zero;