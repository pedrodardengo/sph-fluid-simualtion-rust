use crate::fluid_simulation::cell_manager::CellManager;
use crate::fluid_simulation::collision_manager::CollisionManager;
use crate::fluid_simulation::external_attractor::ExternalAttractor;
use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use crate::fluid_simulation::smoothed_interaction::SmoothedInteraction;
use piston::{Button, Event, Key, Motion, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent, Window, Input};
use rand::Rng;
use rayon::prelude::*;
use vector2d::Vector2D;

pub struct FluidSimulationApp {
    pub particles: Vec<Particle>,
    dynamics_manager: ParticleDynamicsManager,
    smoothed_interaction: SmoothedInteraction,
    external_attractor: ExternalAttractor,
    collision_manager: CollisionManager,
    cell_manager: CellManager,
    pub ups: usize,
    local_densities: Vec<f32>,
    accelerations: Vec<Vector2D<f32>>,
    previous_accelerations: Vec<Vector2D<f32>>,
}

impl FluidSimulationApp {
    pub fn new(box_dimensions: [usize; 2]) -> Self {
        let mut rng = rand::thread_rng();
        let particle_count = 6000;
        let ups: usize = 100;
        let delta_time = 1.0 / ups as f32;
        let pressure_multiplier: f32 = 800000.0;
        let target_density: f32 = 0.00003;
        let smoothing_radius: f32 = 14.0;
        let viscosity: f32 = 0.06;
        let particles: Vec<Particle> = (0..particle_count)
            .map(|index| {
                Particle::new(
                    index,
                    Vector2D::new(
                        rng.gen_range(0.0..(600 as f32)),
                        rng.gen_range(0.0..(box_dimensions[1] as f32)),
                    ),
                )
            })
            .collect();
        let local_densities: Vec<_> = particles.iter().map(|_| 0.001).collect();
        let accelerations: Vec<_> = particles
            .iter()
            .map(|_| Vector2D { x: 0.0, y: 0.0 })
            .collect();
        let previous_accelerations: Vec<_> = accelerations.clone();
        FluidSimulationApp {
            particles,
            ups,
            local_densities,
            accelerations,
            previous_accelerations,
            dynamics_manager: ParticleDynamicsManager::new(true, delta_time),
            smoothed_interaction: SmoothedInteraction::new(
                pressure_multiplier,
                target_density,
                smoothing_radius,
                viscosity,
            ),
            external_attractor: ExternalAttractor::new(),
            collision_manager: CollisionManager::new(box_dimensions),
            cell_manager: CellManager::new(particle_count as i32, box_dimensions, smoothing_radius),
        }
    }

    pub fn update(&mut self) {
        self.particles
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, particle)| {
                //self.dynamics_manager.update_velocity(particle, self.accelerations[index], self.previous_accelerations[index]);
                self.dynamics_manager
                    .update_position(particle, self.accelerations[index]);
                self.collision_manager.apply_boundary_conditions(particle);
            });

        self.cell_manager.update(&mut self.particles);

        self.local_densities
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, density)| {
                let adjacente_particles_indices_iterator = self
                    .cell_manager
                    .get_adjacent_particles_indices(self.particles[index].position);
                *density = self.smoothed_interaction.calculate_density(
                    index,
                    adjacente_particles_indices_iterator,
                    &self.particles,
                )
            });

        self.accelerations
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, acceleration)| {
                let adjacente_particles_indices_iterator = self
                    .cell_manager
                    .get_adjacent_particles_indices(self.particles[index].position);
                let mut new_acceleration = self.smoothed_interaction.calculate_acceleration(
                    index,
                    adjacente_particles_indices_iterator,
                    &self.particles,
                    &self.local_densities,
                );
                new_acceleration += self
                    .external_attractor
                    .get_external_attraction_acceleration(
                        &self.particles[index],
                        self.local_densities[index],
                    );
                *acceleration = new_acceleration;
            });
        //self.previous_accelerations = self.accelerations.clone();
        for i in 0..self.particles.len() {
            self.dynamics_manager.update_velocity(
                &mut self.particles[i],
                self.accelerations[i],
                &mut self.previous_accelerations[i],
            );
        }
    }

    pub fn handle_event(&mut self, event: Event, window: &impl Window) {
        if let Some(Button::Keyboard(Key::G)) = event.press_args() {
            self.dynamics_manager.toggle_gravity();
        }
        if let Some(Button::Keyboard(Key::D)) = event.press_args() {
            self.collision_manager.break_dam();
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            self.external_attractor.active = true;
        }

        match event {
            Event::Input(Input::Move(Motion::MouseCursor(pos)), _) => {
                    self.external_attractor.position = Vector2D::new(pos[0] as f32, pos[1] as f32);
            },
            _ => {}
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            self.external_attractor.active = false;
        }
    }
}
