use vector2d::Vector2D;
use rand::Rng;

pub struct Particle {
    pub position: Vector2D<f32>,
    pub velocity: Vector2D<f32>,
    pub mass: f32,
    pub density: f32,
}

impl Particle {
    pub fn new() -> Self {
      // Generate random position and velocity
      let mut rng = rand::thread_rng();
      let random_position = Vector2D::new(rng.gen_range(0.0..800.0), rng.gen_range(0.0..600.0));
      let random_velocity = Vector2D::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        Particle {
            position: random_position,
            velocity: random_velocity,
            mass: 0.01,
            density: 0.0,
        }
    }

    pub fn speed(&self) -> f32 {
        return self.velocity.length();
    }
}