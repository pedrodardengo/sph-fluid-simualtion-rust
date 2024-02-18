use vector2d::Vector2D;

pub struct CellManager {
  particle_count: i32,
  spatial_lookup: Vec<i32>
}

impl CellManager {

  pub fn new(particle_count: i32) -> Self {
    let spatial_lookup: Vec<i32> = (0..particle_count)
          .map(|_| 0)
          .collect();
    CellManager {
      particle_count,
      spatial_lookup
    }
  }

  pub fn clean_spatial_lookup(&mut self) {
    self.spatial_lookup = (0..self.particle_count)
      .map(|_| 0)
      .collect();
  }

  // fn particle_position_to_cell_coord(self, position: Vector2D<f32>) -> Vector2D<f32> {

  // }

  // fn cell_coord_to_cell_key(self, coord: Vector2D<f32>) -> i32 {
  //   self.spatial_lookup.
  // }
}