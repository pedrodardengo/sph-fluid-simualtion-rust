use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use crate::fluid_simulation::smoothed_interaction::SmoothedInteraction;
use piston::UpdateArgs;
use piston::Button;
use piston::Key;
use piston::Event;
use crate::piston::PressEvent;

pub struct FluidSimulationApp {
  pub particles: Vec<Particle>,
  dynamics_manager: ParticleDynamicsManager,
  smoothed_interaction: SmoothedInteraction
}

impl FluidSimulationApp {

  pub fn new(particle_count: i32) -> Self {
      let particles: Vec<Particle> = (0..particle_count)
          .map(|_| Particle::new())
          .collect();
      let dynamics_manager = ParticleDynamicsManager::new(false);
      let smoothed_interaction = SmoothedInteraction::new(2.5, 1.2, 20.0, 0.001);
      FluidSimulationApp {
          particles,
          dynamics_manager,
          smoothed_interaction
      }
  }

  pub fn update(&mut self, _args: &UpdateArgs) {
    let particles = self.particles.clone();
    for particle in &mut self.particles {
      particle.local_density = self.smoothed_interaction.calculate_density(particle, &particles);
    }
    for particle in &mut self.particles {
      particle.pressure = self.smoothed_interaction.calculate_pressure(particle, &particles);
      particle.viscosity_resistance = self.smoothed_interaction.calculate_viscosity(particle, &particles);
    }
    for particle in &mut self.particles { 
        self.dynamics_manager.execute_dynamics(particle);
    }
  }


  pub fn handle_event(&mut self, event: Event) {
    if let Some(Button::Keyboard(Key::G)) = event.press_args() {
        self.dynamics_manager.toggle_gravity();
    }
  }

}
