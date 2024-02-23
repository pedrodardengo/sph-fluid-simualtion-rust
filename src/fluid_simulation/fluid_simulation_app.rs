use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use crate::fluid_simulation::smoothed_interaction::SmoothedInteraction;
use crate::fluid_simulation::external_attractor::ExternalAttractor;
use crate::fluid_simulation::collision_manager::CollisionManager;
use crate::fluid_simulation::cell_manager::CellManager;
use piston::ReleaseEvent;
use piston::UpdateArgs;
use piston::Button;
use piston::Key;
use piston::MouseButton;
use piston::Event;
use crate::piston::PressEvent;
use vector2d::Vector2D;
use rand::Rng;
use std::time::Instant;

pub struct FluidSimulationApp {
  pub particles: Vec<Particle>,
  dynamics_manager: ParticleDynamicsManager,
  smoothed_interaction: SmoothedInteraction,
  external_attractor: ExternalAttractor,
  collision_manager: CollisionManager,
  cell_manager: CellManager
}

impl FluidSimulationApp {

  pub fn new(box_dimensions: [i32; 2]) -> Self {
      let mut rng = rand::thread_rng();
      let particle_count = 6000;
      let delta_time = 1.0/60.0;
      let pressure_multiplier: f32 = 220000.0;
      let target_density: f32 = 0.00002;
      let smoothing_radius: f32 = 14.0;
      let viscosity: f32 = 0.008;
      let particles = (0..particle_count).map(
        |index| 
        Particle::new(
          index, 
          Vector2D::new(
            rng.gen_range(0.0..(300 as f32)), 
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

  pub fn update(&mut self, _args: &UpdateArgs) {
    for index in 0..self.particles.len() {
      let particle = &mut self.particles[index];
      self.dynamics_manager.update_position(particle);
      self.collision_manager.apply_boundary_conditions(particle);
    }
    self.cell_manager.update(&mut self.particles);
    
    //let start = Instant::now();

    for particle_index in 0..self.particles.len() {
      let adjacente_particles_indices: Vec<usize> = self.cell_manager.get_adjacent_particles(self.particles[particle_index].position);
      self.particles[particle_index].local_density = self.smoothed_interaction.calculate_density(
        particle_index,  
        adjacente_particles_indices,
        &self.particles
      );
    }
    
    //println!("Density {:?}", start.elapsed());

    for particle_index in 0..self.particles.len() {
      let adjacente_particles_indices: Vec<usize> = self.cell_manager.get_adjacent_particles(self.particles[particle_index].position);
      let mut acceleration = self.smoothed_interaction.calculate_pressure(particle_index, &adjacente_particles_indices, &self.particles);
      acceleration += self.smoothed_interaction.calculate_viscosity(particle_index, &adjacente_particles_indices, &self.particles);
      acceleration += self.external_attractor.get_external_attraction_acceleration(&mut self.particles[particle_index]);
      self.particles[particle_index].acceleration = acceleration;
      self.dynamics_manager.update_velocity(&mut self.particles[particle_index]);
    }

  }

  pub fn handle_event(&mut self, event: Event) {
    if let Some(Button::Keyboard(Key::G)) = event.press_args() {
        self.dynamics_manager.toggle_gravity();
    }
    if let Some(Button::Keyboard(Key::D)) = event.press_args() {
      self.collision_manager.break_dam();
    }
    if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
      self.external_attractor.activate(Vector2D::new(400.0 as f32, 100.0 as f32));
    }
    if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
      self.external_attractor.active = false;
    }
  }



}
