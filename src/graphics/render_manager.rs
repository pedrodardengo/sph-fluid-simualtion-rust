use crate::fluid_simulation::particle::Particle;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use std::cmp::max;
use graphics::*;


pub struct RenderManager {
  gl: GlGraphics
}


impl RenderManager {

  pub fn new(gl: GlGraphics) -> Self {
    RenderManager {
        gl
    }
  }


  pub fn render(&mut self, args: &RenderArgs, particles: Vec<Particle>) {
    const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    self.gl.draw(args.viewport(), |c, gl| {
        clear(BLACK_COLOR, gl);
        for particle in particles {
            let color = Self::speed_to_color_gradient(particle.speed());
            ellipse(
                color,
                //[0.0, 0.0, 1.0, 1.0],
                [particle.position.x as f64, particle.position.y as f64, 5.0, 5.0],
                c.transform,
                gl,
            );
        }
    });
  }

  fn speed_to_color_gradient(speed: f32) -> [f32; 4] {
    const MAX_SPEED: f32 = 250.0;
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