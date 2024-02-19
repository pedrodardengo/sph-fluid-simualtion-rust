use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::smothing_kernels::spiky_smoothing_kernel;
use crate::fluid_simulation::smothing_kernels::spiky_smoothing_kernel_derivative;
use crate::fluid_simulation::smothing_kernels::viscosity_smoothing_kernel_second_derivative;
use vector2d::Vector2D;
use rand::Rng;
use super::smothing_kernels::poly6_smoothing_kernel;
use super::smothing_kernels::sb_smoothing_kernel;
use super::smothing_kernels::sb_smoothing_kernel_derivative;

pub struct SmoothedInteraction {
  pressure_multiplier: f32,
  target_density: f32,
  smoothing_radius: f32,
  viscosity: f32
}

impl SmoothedInteraction {

  pub fn new(pressure_multiplier: f32, target_density: f32, smoothing_radius: f32, viscosity: f32) -> Self {
    SmoothedInteraction {
      pressure_multiplier,
      target_density,
      smoothing_radius,
      viscosity
    }
}

  pub fn calculate_acceleration_due_to_pressure(&self, particle: &Particle, particles: &Vec<Particle>) -> Vector2D<f32> {
      let mut rng = rand::thread_rng();
      let mut property_gradient = Vector2D::new(0.0, 0.0);
      for iter_particle in particles {
          if particle.id == iter_particle.id { continue; }
          let mut relative_position = particle.position - iter_particle.position;
          let distance = relative_position.length();
          if distance == 0.0 {
            relative_position = Vector2D::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
          }
          let slope = sb_smoothing_kernel_derivative(distance, self.smoothing_radius);
          if slope == 0.0 { continue;}
          let shared_pressure = self.calculate_shared_pressure(particle.local_density, iter_particle.local_density);
          property_gradient += relative_position.normalise() * shared_pressure * slope * iter_particle.mass / iter_particle.local_density;
      }
      property_gradient / particle.local_density
  }

  pub fn calculate_density(&self, particle: &Particle, particles: &Vec<Particle>) -> f32 {
    let mut density = particle.mass * spiky_smoothing_kernel(0.0, self.smoothing_radius);
    for iter_particle in particles {
        let relative_position = particle.position - iter_particle.position;
        let distance = relative_position.length();
        let influence = sb_smoothing_kernel(distance, self.smoothing_radius);
        if influence == 0.0 { continue;}
        density += iter_particle.mass * influence;
    }
    density
  }

  pub fn calculate_viscosity(&self, particle: &Particle, particles: &Vec<Particle>) -> Vector2D<f32> {
    let mut viscosit_force = Vector2D::new(0.0, 0.0);
    for iter_particle in particles {
      if particle.id == iter_particle.id { continue; }
        let relative_position = particle.position - iter_particle.position;
        let distance = relative_position.length();
        let influence = viscosity_smoothing_kernel_second_derivative(distance, self.smoothing_radius);
        if influence == 0.0 { continue;}
        let relative_speed = iter_particle.velocity - particle.velocity;
        viscosit_force += relative_speed * self.viscosity * iter_particle.mass * influence / iter_particle.local_density;
    }
    viscosit_force / particle.local_density
  }

  fn calculate_shared_pressure(&self, density_a: f32, density_b: f32) -> f32 {
    (self.convert_density_to_pressure(density_a) + self.convert_density_to_pressure(density_b)) / 2.0
  }

  fn convert_density_to_pressure(&self, density: f32) -> f32 {
    self.pressure_multiplier *( density - self.target_density)
  }

}