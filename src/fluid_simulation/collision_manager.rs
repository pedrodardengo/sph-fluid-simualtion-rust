use crate::fluid_simulation::particle::Particle;
pub struct CollisionManager {
    pub box_width: f32,
    pub box_height: f32,
    is_dam_active: bool,
    particle_radius: f32,
}

impl CollisionManager {
    pub fn new(box_dimensions: [usize; 2]) -> Self {
        CollisionManager {
            box_width: box_dimensions[0] as f32,
            box_height: box_dimensions[1] as f32,
            is_dam_active: true,
            particle_radius: 3.0,
        }
    }

    pub fn apply_boundary_conditions(&self, particle: &mut Particle) {
        self.apply_box_boundary(particle);
        self.apply_obstruction(particle);
        if self.is_dam_active {
            self.apply_dam_boundary(particle)
        }
    }

    pub fn break_dam(&mut self) {
        self.is_dam_active = false;
    }

    fn apply_dam_boundary(&self, particle: &mut Particle) {
        if particle.position.x > 600.0 - self.particle_radius {
            particle.position.x = 600.0 - self.particle_radius;
            particle.velocity.x = -particle.velocity.x;
        }
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

    fn apply_obstruction(&self, particle: &mut Particle) {
        let is_beyond_left_side_of_obstruction = particle.position.x > 100.0;
        let is_beyond_right_side_of_obstruction = particle.position.x < 200.0;
        let is_bellow_top_side_of_obstruction = particle.position.y > 400.0;
        let is_above_down_side_of_obstruction = particle.position.y < 750.0;

        if !(is_beyond_left_side_of_obstruction
            && is_beyond_right_side_of_obstruction
            && is_bellow_top_side_of_obstruction
            && is_above_down_side_of_obstruction)
        {
            return;
        }

        let fraction_to_right_side = (particle.position.x - 100.0) / 100.0;
        let fraction_to_left_side = 1.0 - fraction_to_right_side;
        let fraction_to_top_side = (particle.position.y - 400.0) / 350.0;
        let fraction_to_bottom_side = 1.0 - fraction_to_top_side;

        let mut fractions: [(f32, usize); 4] = [
            (fraction_to_right_side, 1),
            (fraction_to_left_side, 2),
            (fraction_to_top_side, 3),
            (fraction_to_bottom_side, 4),
        ];
        fractions.sort_by(|f_a, f_b| f_a.0.partial_cmp(&f_b.0).unwrap());
        let region = fractions
            .iter()
            .max_by(|f_a, f_b| f_a.0.partial_cmp(&f_b.0).unwrap())
            .unwrap()
            .1;
        match region {
            4 => {
                particle.position.y = 400.0;
                particle.velocity.y = -particle.velocity.y
            }
            3 => {
                particle.position.y = 750.0;
                particle.velocity.y = -particle.velocity.y
            }
            2 => {
                particle.position.x = 100.0;
                particle.velocity.x = -particle.velocity.x
            }
            1 => {
                particle.position.x = 200.0;
                particle.velocity.x = -particle.velocity.x
            }
            _ => {}
        }
    }
}
