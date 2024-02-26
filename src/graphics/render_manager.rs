use std::cmp::max;

use minifb::{Window, WindowOptions};

use crate::fluid_simulation::particle::Particle;

pub struct RenderManager {
  pub window: Window,
  buffer: Vec<u32>,
  box_dimensions: [usize; 2]
}


impl RenderManager {

  pub fn new(box_dimensions: [usize; 2]) -> Self {

    let window = Window::new(
      "Test - ESC to exit",
      box_dimensions[0],
      box_dimensions[1],
      WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    RenderManager {
        window,
        buffer: vec![0; box_dimensions[0] * box_dimensions[1]],
        box_dimensions
    }
  }

  pub fn render(&mut self, particles: &Vec<Particle>) {
    self.buffer.iter_mut().for_each(|pixel| *pixel = 0);
    for particle in particles {
      Self::draw_circle(&mut self.buffer, self.box_dimensions[0], particle, 2); // Red color
  }
    self.window
        .update_with_buffer(&self.buffer, self.box_dimensions[0], self.box_dimensions[1])
        .unwrap_or_else(|e| {
            eprintln!("Error updating window: {}", e);
        });
  }

  fn draw_circle(buffer: &mut [u32], width: usize, particle: &Particle, radius: isize) {
    let center_x = particle.position.x as isize;
    let center_y = particle.position.y as isize;
    let color = Self::speed_to_color_gradient(particle.speed());
    for y in (center_y - radius)..=(center_y + radius) {
        for x in (center_x - radius as isize)..=(center_x + radius as isize) {
            let distance_squared = (x - center_x).pow(2) + (y - center_y).pow(2);
            if distance_squared <= radius.pow(2) {
                let index = (y * width as isize + x) as usize;
                buffer[index] = color;
            }
        }
    }
  }

  fn speed_to_color_gradient(speed: f32) -> u32 {
    const MAX_SPEED: f32 = 4000.0;
    let ratio: f32 = speed / MAX_SPEED;
    let normalized = ratio * 256.0 * 4.0;
    let region = (normalized / 256.0) as i32;
    let x = normalized.round() as i32 % 256;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    match region {
        3 => {
            red = 255;
            green = max(255 - x, 0);
            blue = 0;
        }
        2 => {
            red = max(x, 0);
            green = 255;
            blue = 0;
        }
        1 => {
            red = 0;
            green = 255;
            blue = max(255 - x, 0);
        }
        0 => {
            red = 0;
            green = max(x, 0);
            blue = 255;
        }
        _ => {}
    }

    Self::rgb_to_color_value(red, green, blue)
  } 

  fn rgb_to_color_value(red: i32, green: i32, blue: i32) -> u32 {
    (255 << 24) | ((red as u32) << 16) | ((green as u32) << 8) | blue as u32
  }
}