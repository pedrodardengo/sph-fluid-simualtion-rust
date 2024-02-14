use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use piston::UpdateArgs;
use piston::Button;
use piston::Key;
use piston::Event;
use crate::piston::PressEvent;

pub struct FluidSimulationApp {
  pub particles: Vec<Particle>,
  dynamics_manager: ParticleDynamicsManager
}

impl FluidSimulationApp {

  pub fn new(particle_count: i32) -> Self {
      let particles: Vec<Particle> = (0..particle_count)
          .map(|_| Particle::new())
          .collect();
      let dynamics_manager = ParticleDynamicsManager::new(false);

      FluidSimulationApp {
          particles,
          dynamics_manager
      }
  }

  pub fn update(&mut self, _args: &UpdateArgs) {
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
