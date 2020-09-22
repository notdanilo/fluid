pub const REST_DENSITY: f32 = 1000.0; // rest density
pub const GAS_CONSTANT: f32 = 2000.0; // const for equation of state
pub const H: f32 = 16.0; // kernel radius
pub const HSQ: f32 = H*H; // radius^2 for optimization
pub const MASS: f32 = 65.0; // assume all particles have the same mass
pub const VISCOSITY: f32 = 250.0; // viscosity constant
pub use std::f32::consts::PI;
pub const POLY6      : f32 =  315.0 / (65.0 * PI * H * H * H * H * H * H * H * H * H);
pub const SPIKY_GRAD : f32 = -45.0  / (PI * H * H * H * H * H * H);
pub const VISCOSITY_LAP   : f32 =  45.0  / (PI * H * H * H * H * H * H);
pub const EPS: f32 = H; // boundary epsilon
pub const BOUND_DAMPING: f32 = -0.5;
pub const G: f32 = 12000.0 * -9.8;