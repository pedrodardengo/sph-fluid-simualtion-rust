mod fluid_simulation;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use fluid_simulation::fluid_simulation_app::FluidSimulationApp;


fn main() {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;
        const WINDOW_WIDTH: f64 = 800.0;
        const WINDOW_HEIGHT: f64 = 600.0;
        const PARTICLE_COUNT: i32 = 200;

        // Create a Glutin window.
        let mut window: Window = WindowSettings::new("Particle Simulation", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
    
        // Create a new game and run it.
        let mut app = FluidSimulationApp::new(PARTICLE_COUNT, GlGraphics::new(opengl));
    
        let mut events = Events::new(EventSettings::new());
        
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                app.render(&args);
            }
    
            if let Some(args) = e.update_args() {
                app.update(&args);
            }
        }

}