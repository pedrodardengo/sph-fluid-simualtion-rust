use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::smothing_kernels::spiky_smoothing_kernel;
use crate::fluid_simulation::smothing_kernels::viscosity_smoothing_kernel_second_derivative;
use vector2d::Vector2D;
use rand::Rng;
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

  pub fn calculate_pressure(&self, particle_index: usize, adjacent_particle_indices: &Vec<usize>, particles: &Vec<Particle>) -> Vector2D<f32> {
      let mut rng = rand::thread_rng();
      let mut property_gradient = Vector2D::new(0.0, 0.0);
      for iter_particle_index in adjacent_particle_indices {
          if particles[particle_index].id == particles[*iter_particle_index].id { continue; }
          let mut relative_position = particles[particle_index].position - particles[*iter_particle_index].position;
          let distance = relative_position.length();
          if distance == 0.0 {
            relative_position = Vector2D::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
          }
          let slope = sb_smoothing_kernel_derivative(distance, self.smoothing_radius);
          if slope == 0.0 { continue;}
          let shared_pressure = self.calculate_shared_pressure(particles[particle_index].local_density, particles[*iter_particle_index].local_density);
          property_gradient += relative_position.normalise() * shared_pressure * slope * particles[*iter_particle_index].mass / particles[*iter_particle_index].local_density;
      }
      property_gradient / particles[particle_index].local_density
  }

  pub fn calculate_density(&self, particle_index: usize, adjacent_particle_indices: Vec<usize>, particles: &Vec<Particle>) -> f32 {
    let mut density = particles[particle_index].mass * spiky_smoothing_kernel(0.0, self.smoothing_radius);
    for iter_particle_index in adjacent_particle_indices {
        let relative_position = particles[particle_index].position - particles[iter_particle_index].position;
        let distance = relative_position.length();
        let influence = sb_smoothing_kernel(distance, self.smoothing_radius);
        if influence == 0.0 { continue;}
        density += particles[iter_particle_index].mass * influence;
    }
    density
  }

  pub fn calculate_viscosity(&self, particle_index: usize, adjacent_particle_indices: &Vec<usize>, particles: &Vec<Particle>) -> Vector2D<f32> {
    let mut viscosit_force = Vector2D::new(0.0, 0.0);
    for iter_particle_index in adjacent_particle_indices {
      if particles[particle_index].id == particles[*iter_particle_index].id { continue; }
        let relative_position = particles[particle_index].position - particles[*iter_particle_index].position;
        let distance = relative_position.length();
        let influence = viscosity_smoothing_kernel_second_derivative(distance, self.smoothing_radius);
        if influence == 0.0 { continue;}
        let relative_speed = particles[*iter_particle_index].velocity - particles[particle_index].velocity;
        viscosit_force += relative_speed * self.viscosity * particles[*iter_particle_index].mass * influence / particles[*iter_particle_index].local_density;
    }
    viscosit_force / particles[particle_index].local_density
  }

  fn calculate_shared_pressure(&self, density_a: f32, density_b: f32) -> f32 {
    (self.convert_density_to_pressure(density_a) + self.convert_density_to_pressure(density_b)) / 2.0
  }

  fn convert_density_to_pressure(&self, density: f32) -> f32 {
    - self.pressure_multiplier *( density - self.target_density)
  }

}