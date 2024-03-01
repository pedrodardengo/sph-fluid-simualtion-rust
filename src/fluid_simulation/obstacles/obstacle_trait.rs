use crate::fluid_simulation::particle::Particle;

pub trait Obstacle {
    fn apply_obstruction_boundary(&self, particle: &mut Particle) {}
}
