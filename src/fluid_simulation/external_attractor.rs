use crate::fluid_simulation::particle::Particle;
use vector2d::Vector2D;

pub struct ExternalAttractor {
  pub position: Vector2D<f32>,
  pub active: bool,
  radius: f32
}

impl ExternalAttractor {
  pub fn new() -> Self {
    ExternalAttractor {
      position: Vector2D::new(0.0, 0.0),
      active: false,
      radius: 100.0
    }
  }

  pub fn activate(&mut self, position: Vector2D<f32>) -> () {
    self.active = true;
    self.position = position;
  }

  pub fn get_external_attraction_acceleration(&self, particle: &Particle) -> Vector2D<f32> {
    if !self.active {
      return Vector2D::new(0.0, 0.0)
    }
    let vetcor_to_input_point = self.position - particle.position;
    let distance_to_input_point = vetcor_to_input_point.length();
    if distance_to_input_point >= self.radius {
      return Vector2D::new(0.0, 0.0)
    }
    //-(vetcor_to_input_point.normalise() - particle.velocity * (1.0 - distance_to_input_point / self.radius))
    ((vetcor_to_input_point.normalise())/100.0 - particle.velocity * (1.0 - distance_to_input_point / self.radius)/10000.0) / particle.local_density
  }
}
