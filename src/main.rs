pub mod fluid_simulation;
pub mod graphics;

use fluid_simulation::fluid_simulation_app::FluidSimulationApp;
use graphics::render_manager::RenderManager;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use std::time::{Instant, Duration};

extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    const WINDOW_WIDTH: usize = 1000;
    const WINDOW_HEIGHT: usize = 800;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Particle Simulation", [WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut simulation = FluidSimulationApp::new([WINDOW_WIDTH, WINDOW_HEIGHT]);
    let mut renderer = RenderManager::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings {max_fps: 60, ups: simulation.ups as u64, swap_buffers: true, bench_mode: false, lazy: false, ups_reset: 2});

    let num_executions = 600;
    let mut counter = 0;
    let mut total_elapsed_time = Duration::from_secs(0);

    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            let start = Instant::now();
            renderer.render(&args, &simulation.particles);
            total_elapsed_time += start.elapsed();
        }

        if let Some(_) = e.update_args() {
            let start = Instant::now();
            simulation.update();
            total_elapsed_time += start.elapsed();
            counter += 1;
        }

        simulation.handle_event(e);


        if counter >= num_executions {
            // Calculate and print the average time
            let average_time = total_elapsed_time / num_executions as u32;
            println!("Average Time: {:?}", average_time);

            // Reset counters
            counter = 0;
            total_elapsed_time = Duration::from_secs(0);
        }
    }
}