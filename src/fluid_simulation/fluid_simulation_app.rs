use super::obstacles::dam_obstacle::DamObstacle;
use super::obstacles::obstacle_trait::Obstacle;
use super::obstacles::rectangle_obstacle::RectangleObstacle;
use crate::fluid_simulation::cell_manager::CellManager;
use crate::fluid_simulation::config::{Accelerations, Densities, Particles, PARTICLE_COUNT};
use crate::fluid_simulation::external_attractor::ExternalAttractor;
use crate::fluid_simulation::obstacle_collision_manager::ObstacleCollisionManager;
use crate::fluid_simulation::particle::Particle;
use crate::fluid_simulation::particle_dynamics_manager::ParticleDynamicsManager;
use crate::fluid_simulation::smoothed_interaction::SmoothedInteraction;
use piston::{Button, Event, Input, Key, Motion, MouseButton, PressEvent, ReleaseEvent, Window};
use rand::Rng;
use rayon::prelude::*;
use std::time::{Duration, Instant};
use vector2d::Vector2D;
pub struct FluidSimulationApp {
    pub particles: Particles,
    dynamics_manager: ParticleDynamicsManager,
    smoothed_interaction: SmoothedInteraction,
    external_attractor: ExternalAttractor,
    collision_manager: ObstacleCollisionManager,
    cell_manager: CellManager,
    pub ups: usize,
    densities: Densities,
    accelerations: Accelerations,
    previous_accelerations: Accelerations,
    dam_obstacle: DamObstacle,
    rectangle_obstacle: RectangleObstacle,
}

impl FluidSimulationApp {
    pub fn new(box_dimensions: [usize; 2]) -> Self {
        let mut rng = rand::thread_rng();
        let ups: usize = 100;
        let delta_time = 1.0 / ups as f32;
        let pressure_multiplier: f32 = 800000.0;
        let target_density: f32 = 0.00003;
        let smoothing_radius: f32 = 14.0;
        let viscosity: f32 = 0.04;
        let particles: Particles = core::array::from_fn(|index| {
            Particle::new(
                index,
                Vector2D::new(
                    rng.gen_range(0.0..(600 as f32)),
                    rng.gen_range(0.0..(box_dimensions[1] as f32)),
                ),
            )
        });
        let densities: Densities = core::array::from_fn(|_| 0.001);
        let accelerations: Accelerations = core::array::from_fn(|_| Vector2D { x: 0.0, y: 0.0 });
        let rectangle_obstacle: RectangleObstacle = RectangleObstacle::new(box_dimensions);
        let dam_obstacle: DamObstacle = DamObstacle::new(box_dimensions);
        FluidSimulationApp {
            particles,
            ups,
            densities,
            accelerations,
            previous_accelerations: accelerations.clone(),
            dynamics_manager: ParticleDynamicsManager::new(true, delta_time),
            smoothed_interaction: SmoothedInteraction::new(
                pressure_multiplier,
                target_density,
                smoothing_radius,
                viscosity,
            ),
            external_attractor: ExternalAttractor::new(),
            collision_manager: ObstacleCollisionManager::new(box_dimensions),
            cell_manager: CellManager::new(PARTICLE_COUNT as i32, box_dimensions, smoothing_radius),
            dam_obstacle,
            rectangle_obstacle,
        }
    }

    pub fn update(&mut self) {
        //let start = Instant::now();
        self.particles
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, particle)| {
                self.dynamics_manager.update_velocity(
                    particle,
                    self.accelerations[index],
                    self.previous_accelerations[index],
                );
                self.dynamics_manager
                    .update_position(particle, self.accelerations[index]);
                self.collision_manager.apply_boundary_conditions(particle);
                self.dam_obstacle.apply_obstruction_boundary(particle);
                self.rectangle_obstacle.apply_obstruction_boundary(particle);
            });

        self.previous_accelerations = self.accelerations;
        self.cell_manager.update(&mut self.particles);

        self.densities
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
                    &self.densities,
                );
                new_acceleration += self
                    .external_attractor
                    .get_external_attraction_acceleration(
                        &self.particles[index],
                        self.densities[index],
                    );
                *acceleration = new_acceleration;
            });
        //println!("Update: {:?}", start.elapsed());
    }

    pub fn handle_event(&mut self, event: Event, window: &impl Window) {
        if let Some(Button::Keyboard(Key::G)) = event.press_args() {
            self.dynamics_manager.toggle_gravity();
        }
        if let Some(Button::Keyboard(Key::D)) = event.press_args() {
            self.dam_obstacle.break_dam();
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            self.external_attractor.active = true;
        }

        match event {
            Event::Input(Input::Move(Motion::MouseCursor(pos)), _) => {
                self.external_attractor.position = Vector2D::new(pos[0] as f32, pos[1] as f32);
            }
            _ => {}
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            self.external_attractor.active = false;
        }
    }
}
