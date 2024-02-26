use crate::fluid_simulation::particle::Particle;
use vector2d::Vector2D;

pub struct ParticleDynamicsManager {
    is_gravity_on: bool,
    delta_time: f32,
}

impl ParticleDynamicsManager {
    pub fn new(is_gravity_on: bool, delta_time: f32) -> Self {
        ParticleDynamicsManager {
            is_gravity_on,
            delta_time,
        }
    }

    pub fn toggle_gravity(&mut self) {
        self.is_gravity_on = !self.is_gravity_on;
    }

    pub fn execute_dynamics(&self, particle: &mut Particle) {
        self.update_velocity(particle);
        self.update_position(particle);
    }

    pub fn update_velocity(&self, particle: &mut Particle) {
        let gravity: Vector2D<f32> = Vector2D::new(0.0, if self.is_gravity_on { 9.8 } else { 0.0 });
        let acceleration = gravity + particle.acceleration;
        particle.velocity +=
            (acceleration + particle.previous_acceleration) * self.delta_time * 0.5;
        particle.previous_acceleration = acceleration
    }

    pub fn update_position(&self, particle: &mut Particle) {
        let gravity: Vector2D<f32> = Vector2D::new(0.0, if self.is_gravity_on { 9.8 } else { 0.0 });
        let acceleration = gravity + particle.acceleration;
        particle.position +=
            particle.velocity * self.delta_time + acceleration * 0.5 * self.delta_time.powi(2);
    }
}
