use super::smothing_kernels::sb_smoothing_kernel;
use super::smothing_kernels::sb_smoothing_kernel_derivative;
use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::smothing_kernels::spiky_smoothing_kernel;
use crate::fluid_simulation::smothing_kernels::viscosity_smoothing_kernel_second_derivative;
use rand::Rng;
use vector2d::Vector2D;

pub struct SmoothedInteraction {
    pressure_multiplier: f32,
    target_density: f32,
    smoothing_radius: f32,
    viscosity: f32,
}

impl SmoothedInteraction {
    pub fn new(
        pressure_multiplier: f32,
        target_density: f32,
        smoothing_radius: f32,
        viscosity: f32,
    ) -> Self {
        SmoothedInteraction {
            pressure_multiplier,
            target_density,
            smoothing_radius,
            viscosity,
        }
    }

    pub fn calculate_acceleration(
        &self,
        particle_index: usize,
        adjacent_particle_indices: impl Iterator<Item = usize>,
        particles: &Vec<Particle>,
        local_densities: &Vec<f32>,
    ) -> Vector2D<f32> {
        let mut acceleration = Vector2D::new(0.0, 0.0);
        for iter_particle_index in adjacent_particle_indices {
            if particles[particle_index].id == particles[iter_particle_index].id {
                continue;
            }
            let mut relative_position =
                particles[particle_index].position - particles[iter_particle_index].position;
            let distance = relative_position.length();
            if distance == 0.0 {
                let mut rng = rand::thread_rng();
                relative_position =
                    Vector2D::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            }
            // pressure
            let slope = sb_smoothing_kernel_derivative(distance, self.smoothing_radius);
            if slope == 0.0 {
                continue;
            }
            let shared_pressure = self.calculate_shared_pressure(
                local_densities[particle_index],
                local_densities[iter_particle_index],
            );
            acceleration += relative_position.normalise()
                * shared_pressure
                * slope
                * particles[iter_particle_index].mass
                / local_densities[iter_particle_index];

            // vicosity
            let relative_speed =
                particles[iter_particle_index].velocity - particles[particle_index].velocity;
            let influence =
                viscosity_smoothing_kernel_second_derivative(distance, self.smoothing_radius);
            acceleration +=
                relative_speed * self.viscosity * particles[iter_particle_index].mass * influence
                    / local_densities[iter_particle_index];
        }
        acceleration / local_densities[particle_index]
    }

    pub fn calculate_density(
        &self,
        particle_index: usize,
        adjacent_particle_indices: impl Iterator<Item = usize>,
        particles: &Vec<Particle>,
    ) -> f32 {
        let mut density =
            particles[particle_index].mass * spiky_smoothing_kernel(0.0, self.smoothing_radius);
        for iter_particle_index in adjacent_particle_indices {
            let relative_position =
                particles[particle_index].position - particles[iter_particle_index].position;
            let distance = relative_position.length();
            let influence = sb_smoothing_kernel(distance, self.smoothing_radius);
            if influence == 0.0 {
                continue;
            }
            density += particles[iter_particle_index].mass * influence;
        }
        density
    }

    fn calculate_shared_pressure(&self, density_a: f32, density_b: f32) -> f32 {
        (self.convert_density_to_pressure(density_a) + self.convert_density_to_pressure(density_b))
            / 2.0
    }

    fn convert_density_to_pressure(&self, density: f32) -> f32 {
        -self.pressure_multiplier * (density - self.target_density)
    }
}
