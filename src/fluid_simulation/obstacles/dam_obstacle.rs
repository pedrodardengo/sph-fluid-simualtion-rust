use super::obstacle_trait::Obstacle;
use crate::fluid_simulation::particle::Particle;

pub struct DamObstacle {
    x_position: f32,
    is_dam_active: bool,
    box_dimensions: [usize; 2],
}

impl Obstacle for DamObstacle {
    fn apply_obstruction_boundary(&self, particle: &mut Particle) {
        if !self.is_dam_active {
            return;
        }
        if particle.position.x > self.x_position {
            particle.position.x = self.x_position;
            particle.velocity.x = -particle.velocity.x;
        }
    }
}

impl DamObstacle {
    pub fn new(box_dimensions: [usize; 2]) -> Self {
        DamObstacle {
            x_position: 600.0,
            box_dimensions,
            is_dam_active: true,
        }
    }

    pub fn break_dam(&mut self) {
        self.is_dam_active = false;
    }
}
