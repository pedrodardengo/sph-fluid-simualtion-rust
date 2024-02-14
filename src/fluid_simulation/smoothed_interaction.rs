use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::smothing_kernels::spiky_smoothing_kernel;
use crate::fluid_simulation::smothing_kernels::spiky_smoothing_kernel_derivative;
use crate::fluid_simulation::smothing_kernels::viscosity_smoothing_kernel_second_derivative;
use vector2d::Vector2D;

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

  pub fn calculate_pressure(&self, particle: &Particle, particles: &Vec<Particle>) -> Vector2D<f32> {
      let mut property_gradient = Vector2D::new(0.0, 0.0);
      for iter_particle in particles {
          let relative_position = particle.position - iter_particle.position;
          let distance = relative_position.length();

          if distance == 0.0 {
              continue;
          }

          let relative_position_unitary = relative_position / distance;
          let slope = spiky_smoothing_kernel_derivative(distance, self.smoothing_radius);
          let shared_pressure = self.calculate_shared_pressure(particle.local_density, iter_particle.local_density);
          property_gradient += relative_position_unitary * shared_pressure * slope * iter_particle.mass / iter_particle.local_density;
      }
      property_gradient
  }

  pub fn calculate_density(&self, particle: &Particle, particles: &Vec<Particle>) -> f32 {
    let mut density = 0.0;

    for iter_particle in particles {
        let relative_position = particle.position - iter_particle.position;
        let distance = relative_position.length();
        let influence = spiky_smoothing_kernel(distance, self.smoothing_radius);
        density += iter_particle.mass * influence;
    }

    density
  }

  pub fn calculate_viscosity(&self, particle: &Particle, particles: &Vec<Particle>) -> Vector2D<f32> {
    let mut viscosit_force = Vector2D::new(0.0, 0.0);

    for iter_particle in particles {
        let relative_position = particle.position - iter_particle.position;
        let distance = relative_position.length();
        let relative_speed = iter_particle.velocity - particle.velocity;
        let influence = viscosity_smoothing_kernel_second_derivative(distance, self.smoothing_radius);
        viscosit_force += relative_speed * self.viscosity * iter_particle.mass * influence / particle.local_density;
    }
    viscosit_force
  }

  fn calculate_shared_pressure(&self, density_a: f32, density_b: f32) -> f32 {
    (self.convert_density_to_pressure(density_a) + self.convert_density_to_pressure(density_b)) / 2.0
  }

  fn convert_density_to_pressure(&self, density: f32) -> f32 {
    let density_difference = density - self.target_density;
    let pressure = density_difference * self.pressure_multiplier;
    pressure
  }

}