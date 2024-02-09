use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;


pub struct FluidSimulationApp {
  gl: GlGraphics, // OpenGL drawing backend.
  particles: Vec<Particle>,
  dynamics_manager: ParticleDynamicsManager
}

impl FluidSimulationApp {

  pub fn new(particle_count: i32, gl: GlGraphics) -> Self {
      let particles: Vec<Particle> = (0..particle_count)
          .map(|_| Particle::new())
          .collect();
      let dynamics_manager = ParticleDynamicsManager::new(false);

      FluidSimulationApp {
          gl,
          particles,
          dynamics_manager
      }
  }

  pub fn render(&mut self, args: &RenderArgs) {
      use graphics::*;
      const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
      const BLUE_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];


      self.gl.draw(args.viewport(), |c, gl| {

          clear(BLACK_COLOR, gl);

          for particle in &mut self.particles {
              ellipse(
                  BLUE_COLOR,
                  [particle.position.x as f64, particle.position.y as f64, 5.0, 5.0], // Particle size
                  c.transform,
                  gl,
              );
          }

      });
  }

  pub fn update(&mut self, _args: &UpdateArgs) {
      for particle in &mut self.particles {
          self.dynamics_manager.execute_dynamics(particle);
      }
  }
}