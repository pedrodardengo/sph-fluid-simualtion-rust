use crate::fluid_simulation::particle::Particle;
pub struct ObstacleCollisionManager {
    pub box_width: f32,
    pub box_height: f32,
    particle_radius: f32,
}

impl ObstacleCollisionManager {
    pub fn new(box_dimensions: [usize; 2]) -> Self {
        ObstacleCollisionManager {
            box_width: box_dimensions[0] as f32,
            box_height: box_dimensions[1] as f32,
            particle_radius: 3.0,
        }
    }

    pub fn apply_boundary_conditions(&self, particle: &mut Particle) {
        self.apply_box_boundary(particle);
    }

    fn apply_box_boundary(&self, particle: &mut Particle) {
        let distance_from_wall = self.particle_radius * 3.0;
        if particle.position.x < distance_from_wall {
            particle.position.x = distance_from_wall;
            particle.velocity.x = -particle.velocity.x;
        }
        if particle.position.x > self.box_width - distance_from_wall {
            particle.position.x = self.box_width - distance_from_wall;
            particle.velocity.x = -particle.velocity.x;
        }
        if particle.position.y < distance_from_wall {
            particle.position.y = distance_from_wall;
            particle.velocity.y = -particle.velocity.y;
        }
        if particle.position.y > self.box_height - distance_from_wall {
            particle.position.y = self.box_height - distance_from_wall;
            particle.velocity.y = -particle.velocity.y;
        }
    }
}
