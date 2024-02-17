use crate::fluid_simulation::particle::Particle;
pub struct CollisionManager {
  pub box_width: f32,
  pub box_height: f32
}

impl CollisionManager {
  pub fn new(box_width: f32, box_height: f32) -> Self {
    CollisionManager {
      box_width,
      box_height
    }
  }

  pub fn apply_boundary_conditions(&self, particle: &mut Particle) {
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