

use opengl_graphics::GlGraphics;
use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use piston::RenderArgs;
use piston::UpdateArgs;
use piston::Button;
use piston::Key;
use piston::Event;
use crate::piston::PressEvent;
use std::cmp::max;
use graphics::*;

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
    const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    self.gl.draw(args.viewport(), |c, gl| {
        clear(BLACK_COLOR, gl);
        for particle in &self.particles {
            let color = Self::speed_to_color_gradient(particle.speed());
            ellipse(
              color,
                [particle.position.x as f64, particle.position.y as f64, 5.0, 5.0],
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


  pub fn handle_event(&mut self, event: Event) {
    if let Some(Button::Keyboard(Key::G)) = event.press_args() {
        self.dynamics_manager.toggle_gravity();
    }
  }

  fn speed_to_color_gradient(speed: f32) -> [f32; 4] {
    const MAX_SPEED: f32 = 10.0;
    let ratio: f32 = speed / MAX_SPEED;
    let normalized = (ratio * 256.0 * 4.0) as i32;
    let region = (normalized / 256) as i32;
    let x = normalized % 256;
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    match region {
        3 => {
            r = 1.0;
            g = (max(255 - x, 0) as f32)/ 255.0;
            b = 0.0;
        }
        2 => {
            r = (max(x, 0) as f32) / 255.0;
            g = 1.0;
            b = 0.0;
        }
        1 => {
            r = 0.0;
            g = 1.0;
            b = (max(255 - x, 0) as f32) / 255.0;
        }
        0 => {
            r = 0.0;
            g = (max(x, 0) as f32) / 255.0;
            b = 1.0;
        }
        _ => {}
    }

    [r, g, b, 1.0]
  } 
}
