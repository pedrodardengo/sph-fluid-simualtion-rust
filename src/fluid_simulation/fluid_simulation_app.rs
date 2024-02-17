use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use crate::fluid_simulation::smoothed_interaction::SmoothedInteraction;
use piston::ReleaseEvent;
use piston::UpdateArgs;
use piston::Button;
use piston::Key;
use piston::MouseButton;
use piston::Event;
use crate::piston::PressEvent;
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

  pub fn get_external_attraction_force(&self, particle: &Particle) -> Vector2D<f32> {
    if !self.active {
      return Vector2D::new(0.0, 0.0)
    }
    let vetcor_to_input_point = self.position - particle.position;
    let distance_to_input_point = vetcor_to_input_point.length();
    if distance_to_input_point >= self.radius {
      return Vector2D::new(0.0, 0.0)
    }
    //-(vetcor_to_input_point.normalise() - particle.velocity * (1.0 - distance_to_input_point / self.radius))
    (vetcor_to_input_point.normalise())/50.0 - particle.velocity * (1.0 - distance_to_input_point / self.radius)/10000.0
  }
}


pub struct FluidSimulationApp {
  pub particles: Vec<Particle>,
  dynamics_manager: ParticleDynamicsManager,
  smoothed_interaction: SmoothedInteraction,
  external_attractor: ExternalAttractor
}

impl FluidSimulationApp {

  pub fn new() -> Self {
      let particle_count = 1000;
      let delta_time = 1.0/120.0;
      let pressure_multiplier: f32 = 8.0;
      let target_density: f32 = 0.01;
      let smoothing_radius: f32 = 10.0;
      let viscosity: f32 = 0.01;
      
      let particles: Vec<Particle> = (0..particle_count)
          .map(|_| Particle::new())
          .collect();
      let dynamics_manager = ParticleDynamicsManager::new(false, delta_time);
      let smoothed_interaction = SmoothedInteraction::new(pressure_multiplier, target_density, smoothing_radius, viscosity);
      FluidSimulationApp {
          particles,
          dynamics_manager,
          smoothed_interaction,
          external_attractor: ExternalAttractor::new()
      }
  }

  pub fn update(&mut self, _args: &UpdateArgs) {
    for particle in &mut self.particles { 
      self.dynamics_manager.update_position(particle);
      self.dynamics_manager.apply_boundary_conditions(particle);
    }
    let mut particles = self.particles.clone();

    for particle in &mut self.particles {
      particle.local_density = self.smoothed_interaction.calculate_density(particle, &particles);
    }
    particles = self.particles.clone();
    for particle in &mut self.particles {
      particle.previous_acceleration = particle.pressure;
      particle.pressure = self.smoothed_interaction.calculate_pressure(particle, &particles);
      particle.pressure += self.smoothed_interaction.calculate_viscosity(particle, &particles);
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
    if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
      self.external_attractor.activate(Vector2D::new(400.0 as f32, 100.0 as f32));
    }
    if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
      self.external_attractor.active = false;
    }
  }

}
