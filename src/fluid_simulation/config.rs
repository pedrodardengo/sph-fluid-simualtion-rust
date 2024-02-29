use crate::fluid_simulation::particle::Particle;
use vector2d::Vector2D;

pub const PARTICLE_COUNT: usize = 15000;
pub type Particles = [Particle; PARTICLE_COUNT];
pub type Accelerations = [Vector2D<f32>; PARTICLE_COUNT];
pub type Densities = [f32; PARTICLE_COUNT];
