pub mod fluid_simulation;
pub mod graphics;

extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use fluid_simulation::fluid_simulation_app::FluidSimulationApp;
use graphics::render_manager::RenderManager;
use std::time::Instant;

fn main() {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;
        const WINDOW_WIDTH: f64 = 1000.0;
        const WINDOW_HEIGHT: f64 = 800.0;

        // Create a Glutin window.
        let mut window: Window = WindowSettings::new("Particle Simulation", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
    
        // Create a new game and run it.
        let mut simulation = FluidSimulationApp::new([WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32]);
        let mut renderer = RenderManager::new(GlGraphics::new(opengl));

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                //let start = Instant::now();
                renderer.render(&args, &simulation.particles);
                //println!("Render {:?}", start.elapsed());
            }
    
            if let Some(args) = e.update_args() {
                //let start = Instant::now();
                simulation.update(&args);
                //println!("Update {:?}", start.elapsed());
            }
            simulation.handle_event(e);
        }
}