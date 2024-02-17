use vector2d::Vector2D;
use rand::Rng;

#[derive(Clone)]
pub struct Particle {
    pub position: Vector2D<f32>,
    pub velocity: Vector2D<f32>,
    pub mass: f32,
    pub local_density: f32,
    pub pressure: Vector2D<f32>,
    pub previous_acceleration: Vector2D<f32>
}

impl Particle {
    pub fn new() -> Self {
      // Generate random position and velocity
      let mut rng = rand::thread_rng();
      let position = Vector2D::new(rng.gen_range(0.0..400.0), rng.gen_range(0.0..600.0));
      let velocity = Vector2D::new(0.0,0.0);
        Particle {
            position,
            velocity,
            mass: 0.001,
            local_density: 0.01,
            pressure: Vector2D::new(0.0, 0.0),
            previous_acceleration: Vector2D::new(0.0, 0.0),
        }
    }

    pub fn get_predicted_position(&self) -> Vector2D<f32> {
        self.position + self.velocity * 1.0/120.0
    }

    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }
}