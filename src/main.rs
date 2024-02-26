pub mod fluid_simulation;
pub mod graphics;

use fluid_simulation::fluid_simulation_app::FluidSimulationApp;
use graphics::render_manager::RenderManager;
use minifb::Key;
use std::time::Instant;

fn main() {
    let box_dimensions: [usize; 2] = [1000, 800];
    let mut simulation = FluidSimulationApp::new(box_dimensions);

    let mut render_manager: RenderManager = RenderManager::new(box_dimensions);

    // Limit to max ~60 fps update rate
    render_manager
        .window
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while render_manager.window.is_open() && !render_manager.window.is_key_down(Key::Escape) {
        simulation.handle_event(&render_manager.window);

        let start = Instant::now();
        simulation.update(); // Update particle positions in state
        println!("Update {:?}", start.elapsed());

        //let start = Instant::now();
        render_manager.render(&simulation.particles);
        //println!("Render {:?}", start.elapsed());
    }
}
