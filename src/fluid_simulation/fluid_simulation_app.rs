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



pub struct FluidSimulationApp {
  pub particles: Vec<Particle>,
  dynamics_manager: ParticleDynamicsManager,
  smoothed_interaction: SmoothedInteraction,
  external_attractor: ExternalAttractor,
  collision_manager: CollisionManager,
  cell_manager: CellManager
}

impl FluidSimulationApp {

  pub fn new(window_width: f32, window_height: f32) -> Self {
      let particle_count = 2000;
      let delta_time = 1.0/460.0;
      let pressure_multiplier: f32 = 1600.4;
      let target_density: f32 = 0.001;
      let smoothing_radius: f32 = 8.0;
      let viscosity: f32 = 0.00;
      
      let particles: Vec<Particle> = (0..particle_count)
          .map(|index| Particle::new(index))
          .collect();
      let dynamics_manager = ParticleDynamicsManager::new(true, delta_time);
      let smoothed_interaction = SmoothedInteraction::new(pressure_multiplier, target_density, smoothing_radius, viscosity);
      FluidSimulationApp {
          particles,
          dynamics_manager,
          smoothed_interaction,
          external_attractor: ExternalAttractor::new(),
          collision_manager: CollisionManager::new(window_width, window_height),
          cell_manager: CellManager::new(particle_count)
      }
  }

  pub fn update(&mut self, _args: &UpdateArgs) {
    for particle in &mut self.particles { 
      self.dynamics_manager.update_position(particle);
      self.collision_manager.apply_boundary_conditions(particle);
    }


    let mut particles = self.particles.clone();

    for particle in &mut self.particles {
      particle.local_density = self.smoothed_interaction.calculate_density(particle, &particles);
    }
    particles = self.particles.clone();
    for particle in &mut self.particles {
      particle.previous_acceleration = particle.pressure;
      particle.pressure = self.smoothed_interaction.calculate_pressure(particle, &particles);
      //particle.pressure += self.smoothed_interaction.calculate_viscosity(particle, &particles);
      particle.pressure += self.external_attractor.get_external_attraction_force(particle)
    }

    for particle in &mut self.particles { 
        self.dynamics_manager.update_velocity(particle);
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
