use crate::fluid_simulation::particle::Particle;
pub struct CollisionManager {
  pub box_width: f32,
  pub box_height: f32,
  is_dam_active: bool 
}

impl CollisionManager {
  pub fn new(box_width: f32, box_height: f32) -> Self {
    CollisionManager {
      box_width,
      box_height,
      is_dam_active: true
    }
  }

  pub fn apply_boundary_conditions(&self, particle: &mut Particle) {
    self.apply_box_boundary(particle);
    if self.is_dam_active {
      self.apply_dam_boundary(particle)
    }    
  }

  pub fn break_dam(&mut self) {
    self.is_dam_active = false;
  }

  fn apply_dam_boundary(&self, particle: &mut Particle) {
    if particle.position.x > 200.0 - 3.0 {
      particle.position.x = 200.0 - 3.0;
      particle.velocity.x = -particle.velocity.x;
    }
  }

  fn apply_box_boundary(&self, particle: &mut Particle) {
    if particle.position.x < 3.0 {
      particle.position.x = 3.0;
      particle.velocity.x = -particle.velocity.x;
    }
    if particle.position.x > self.box_width - 3.0 {
      particle.position.x = self.box_width - 3.0;
      particle.velocity.x = -particle.velocity.x;
    }
    if particle.position.y < 3.0 {
      particle.position.y = 3.0;
      particle.velocity.y = -particle.velocity.y;
    }
    if particle.position.y > self.box_height - 3.0 {
      particle.position.y = self.box_height - 3.0;
      particle.velocity.y = -particle.velocity.y;
    }
  }
}