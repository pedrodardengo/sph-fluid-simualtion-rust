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
        self.update_position(particle);
        self.update_velocity(particle);
    }

    fn update_position(&self, particle: &mut Particle) {
      particle.position.x += particle.velocity.x;
      particle.position.y += particle.velocity.y;
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

    fn update_velocity(&self, particle: &mut Particle) {
      let gravity: f32 = if self.is_gravity_on { 0.1 } else { 0.0 };
      let calculated_resistance = self.calculate_resistance(particle);
      
      particle.velocity.y += gravity + calculated_resistance.y;
      particle.velocity.x += calculated_resistance.x;
      
      if particle.velocity.y.abs() < 0.00001 {
          particle.velocity.y = 0.0;
      }
      
      if particle.velocity.x.abs() < 0.00001 {
          particle.velocity.x = 0.0;
      }
  }

    fn calculate_resistance(&self, particle: &Particle) -> Vector2D<f32> {
      const RESISTANCE_STRENGTH: f32 = 0.003;
      return Vector2D::new(-particle.velocity.x * RESISTANCE_STRENGTH, -particle.velocity.y * RESISTANCE_STRENGTH);
    }
}