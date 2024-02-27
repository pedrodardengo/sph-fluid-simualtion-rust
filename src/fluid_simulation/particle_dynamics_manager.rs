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

    pub fn update_velocity(&self, particle: &mut Particle, acceleration: Vector2D<f32>, previous_acceleration: &mut Vector2D<f32>) {
        let gravity: Vector2D<f32> =
            Vector2D::new(0.0, if self.is_gravity_on { 490.0 } else { 0.0 });
        let acceleration = gravity + acceleration;
        particle.velocity +=
            (acceleration + *previous_acceleration) * self.delta_time * 0.5;
        *previous_acceleration = acceleration;
    }

    pub fn update_position(&self, particle: &mut Particle, acceleration: Vector2D<f32>) {
        let gravity: Vector2D<f32> =
            Vector2D::new(0.0, if self.is_gravity_on { 490.0 } else { 0.0 });
        let acceleration = gravity + acceleration;
        particle.position +=
            particle.velocity * self.delta_time + acceleration * 0.5 * self.delta_time.powi(2);
    }
}
