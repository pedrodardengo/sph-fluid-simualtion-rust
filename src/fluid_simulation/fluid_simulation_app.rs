use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use crate::fluid_simulation::smoothed_interaction::SmoothedInteraction;
use crate::fluid_simulation::external_attractor::ExternalAttractor;
use crate::fluid_simulation::collision_manager::CollisionManager;
use crate::fluid_simulation::cell_manager::CellManager;
use minifb::{Key, MouseButton, Window};
use vector2d::Vector2D;
use rand::Rng;
use rayon::prelude::*;

pub struct FluidSimulationApp {
  pub particles: Vec<Particle>,
  dynamics_manager: ParticleDynamicsManager,
  smoothed_interaction: SmoothedInteraction,
  external_attractor: ExternalAttractor,
  collision_manager: CollisionManager,
  cell_manager: CellManager
}

impl FluidSimulationApp {

  pub fn new(box_dimensions: [usize; 2]) -> Self {
      let mut rng = rand::thread_rng();
      let particle_count = 6000;
      let delta_time = 1.0/60.0;
      let pressure_multiplier: f32 = 220000.0;
      let target_density: f32 = 0.00002;
      let smoothing_radius: f32 = 14.0;
      let viscosity: f32 = 0.018;
      let particles = (0..particle_count).map(
        |index| 
        Particle::new(
          index, 
          Vector2D::new(
            rng.gen_range(0.0..(600 as f32)), 
            rng.gen_range(0.0..(box_dimensions[1] as f32))
          )
        )
      ).collect();
      FluidSimulationApp {
          particles,
          dynamics_manager: ParticleDynamicsManager::new(true, delta_time),
          smoothed_interaction: SmoothedInteraction::new(pressure_multiplier, target_density, smoothing_radius, viscosity),
          external_attractor: ExternalAttractor::new(),
          collision_manager: CollisionManager::new(box_dimensions),
          cell_manager: CellManager::new(particle_count as i32, box_dimensions, smoothing_radius)
      }
  }

  pub fn update(&mut self) {

    self.particles.par_iter_mut().for_each(|particle| {
      self.dynamics_manager.update_position(particle);
      self.collision_manager.apply_boundary_conditions(particle);
    });

    self.cell_manager.update(&mut self.particles);
    
   let densities: Vec<f32> = (0..self.particles.len()).into_par_iter().map(|particle_index| {
      let adjacente_particles_indices_iterator = self.cell_manager.get_adjacent_particles_indices(self.particles[particle_index].position);
      let density = self.smoothed_interaction.calculate_density(
          particle_index,
          adjacente_particles_indices_iterator,
          &self.particles
      );
      density
    }).collect();

    self.particles.par_iter_mut().for_each(|particle| {
      particle.local_density = densities[particle.id];
    });

    let accelerations: Vec<Vector2D<f32>> = (0..self.particles.len()).into_par_iter().map(|particle_index| {
      let adjacente_particles_indices_iterator = self.cell_manager.get_adjacent_particles_indices(self.particles[particle_index].position);
      let mut acceleration = self.smoothed_interaction.calculate_acceleration(
        particle_index, 
        adjacente_particles_indices_iterator, 
        &self.particles
      );
      acceleration += self.external_attractor.get_external_attraction_acceleration(&self.particles[particle_index]);
      acceleration
    }).collect();

    self.particles.par_iter_mut().for_each(|particle| {
      particle.acceleration = accelerations[particle.id];
      self.dynamics_manager.update_velocity(particle);
    });

  }

  pub fn handle_event(&mut self, window: &Window) {
    if window.is_key_down(Key::G) {
        self.dynamics_manager.toggle_gravity();
    }
    if window.is_key_down(Key::D) {
      self.collision_manager.break_dam();
    }
    if window.get_mouse_down(MouseButton::Left) {
      let mouse_position = window.get_mouse_pos(minifb::MouseMode::Discard);
      if let Some(mouse_position) = mouse_position {
        self.external_attractor.activate(Vector2D::new(mouse_position.0, mouse_position.1));
      }
    }
    if !window.get_mouse_down(MouseButton::Left) {
      self.external_attractor.active = false;
    }
  }



}
