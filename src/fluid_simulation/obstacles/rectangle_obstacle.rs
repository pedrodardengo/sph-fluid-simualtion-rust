use super::obstacle_trait::Obstacle;
use crate::fluid_simulation::particle::Particle;
use vector2d::Vector2D;

pub struct RectangleObstacle {
    top_left_corner: Vector2D<f32>,
    bottom_right_corner: Vector2D<f32>,
    dimensions: [f32; 2],
    bounding_box_dimensions: [usize; 2],
}

impl Obstacle for RectangleObstacle {
    fn apply_obstruction_boundary(&self, particle: &mut Particle) {
        let is_beyond_left_side_of_obstruction = particle.position.x > self.top_left_corner.x;
        let is_beyond_right_side_of_obstruction = particle.position.x < self.bottom_right_corner.x;
        let is_bellow_top_side_of_obstruction = particle.position.y > self.top_left_corner.y;
        let is_above_down_side_of_obstruction = particle.position.y < self.bottom_right_corner.y;

        if !(is_beyond_left_side_of_obstruction
            && is_beyond_right_side_of_obstruction
            && is_bellow_top_side_of_obstruction
            && is_above_down_side_of_obstruction)
        {
            return;
        }

        let fraction_to_right_side =
            (particle.position.x - self.top_left_corner.x) / self.dimensions[0];
        let fraction_to_left_side = 1.0 - fraction_to_right_side;
        let fraction_to_top_side =
            (particle.position.y - self.top_left_corner.y) / self.dimensions[1];
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
                particle.position.y = self.top_left_corner.y;
                particle.velocity.y = -particle.velocity.y
            }
            3 => {
                particle.position.y = self.bottom_right_corner.y;
                particle.velocity.y = -particle.velocity.y
            }
            2 => {
                particle.position.x = self.top_left_corner.x;
                particle.velocity.x = -particle.velocity.x
            }
            1 => {
                particle.position.x = self.bottom_right_corner.x;
                particle.velocity.x = -particle.velocity.x
            }
            _ => {}
        }
    }
}

impl RectangleObstacle {
    pub fn new(bounding_box_dimensions: [usize; 2]) -> Self {
        let top_left_corner: Vector2D<f32> = Vector2D::new(100.0, 400.0);
        let bottom_right_corner: Vector2D<f32> = Vector2D::new(200.0, 750.0);
        let dimensions: [f32; 2] = [
            bottom_right_corner.x - top_left_corner.x,
            bottom_right_corner.y - top_left_corner.y,
        ];
        RectangleObstacle {
            top_left_corner,
            bottom_right_corner,
            dimensions,
            bounding_box_dimensions,
        }
    }

    pub fn move_up(&mut self) {
        if self.top_left_corner.y < 0.0 {
            self.top_left_corner += Vector2D::new(0.0, 1.0);
            self.bottom_right_corner += Vector2D::new(0.0, -1.0);
        }
    }

    pub fn move_down(&mut self) {
        if self.bottom_right_corner.y < self.bounding_box_dimensions[1] as f32 {
            self.top_left_corner += Vector2D::new(0.0, -1.0);
            self.bottom_right_corner += Vector2D::new(0.0, 1.0);
        }
    }

    pub fn move_right(&mut self) {
        if self.bottom_right_corner.x < self.bounding_box_dimensions[0] as f32 {
            self.top_left_corner += Vector2D::new(-1.0, 0.0);
            self.bottom_right_corner += Vector2D::new(1.0, 0.0);
        }
    }

    pub fn move_left(&mut self) {
        if self.top_left_corner.x < 0.0 {
            self.top_left_corner += Vector2D::new(1.0, 0.0);
            self.bottom_right_corner += Vector2D::new(-1.0, 0.0);
        }
    }
}
