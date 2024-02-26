use vector2d::Vector2D;

#[derive(Clone)]
pub struct Particle {
    pub id: usize,
    pub cell_key: usize,
    pub position: Vector2D<f32>,
    pub velocity: Vector2D<f32>,
    pub mass: f32,
    pub local_density: f32,
    pub acceleration: Vector2D<f32>,
    pub previous_acceleration: Vector2D<f32>,
}

impl Particle {
    pub fn new(id: usize, position: Vector2D<f32>) -> Self {
        // Generate random position and velocity
        let velocity = Vector2D::new(0.0, 0.0);
        Particle {
            id,
            cell_key: 0,
            position,
            velocity,
            mass: 0.0008,
            local_density: 0.01,
            acceleration: Vector2D::new(0.0, 0.0),
            previous_acceleration: Vector2D::new(0.0, 0.0),
        }
    }

    pub fn get_predicted_position(&self) -> Vector2D<f32> {
        self.position + self.velocity * 1.0 / 120.0
    }

    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }
}
