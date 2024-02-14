use vector2d::Vector2D;
use crate::fluid_simulation::particle::Particle;

pub struct ParticleDynamicsManager {
    is_gravity_on: bool,
}

impl ParticleDynamicsManager {
    pub fn new(is_gravity_on: bool) -> Self {
        ParticleDynamicsManager { is_gravity_on }
    }

    pub fn toggle_gravity(&mut self) {
        self.is_gravity_on = !self.is_gravity_on;
    }

    pub fn execute_dynamics(&self, particle: &mut Particle) {
        self.update_velocity(particle);
        self.update_position(particle);
    }

    fn update_velocity(&self, particle: &mut Particle) {
      let gravity: Vector2D<f32> =  Vector2D::new(0.0, if self.is_gravity_on { 980.0 } else { 0.0 });
      particle.velocity += (gravity + (particle.pressure + particle.viscosity_resistance)/particle.local_density) * 0.015;
    }

    fn update_position(&self, particle: &mut Particle) {
      particle.position += particle.velocity * 0.015;
      if particle.position.x < 3.0 {
        particle.position.x = 3.0;
        particle.velocity.x = -particle.velocity.x;
      }
      if particle.position.x > 800.0 - 3.0 {
        particle.position.x = 800.0 - 3.0;
        particle.velocity.x = -particle.velocity.x;
      }
      if particle.position.y < 3.0 {
        particle.position.y = 3.0;
        particle.velocity.y = -particle.velocity.y;
      }
      if particle.position.y > 600.0 - 3.0 {
        particle.position.y = 600.0 - 3.0;
        particle.velocity.y = -particle.velocity.y;
      }
    }
}