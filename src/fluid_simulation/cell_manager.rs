use crate::fluid_simulation::particle::Particle;
use vector2d::Vector2D;

pub struct CellManager {
  particle_count: i32,
  spatial_lookup: Vec<(usize, usize)>,
  starting_indices: Vec<usize>,
  number_of_columns: i32,
  number_of_rows: i32,
  cell_size: f32,
  number_of_cells: i32
}

impl CellManager {

  pub fn new(particle_count: i32, box_dimensions: [i32; 2], smoothing_radius: f32) -> Self {
    let cell_size = 2.0 * smoothing_radius;
    let number_of_columns = (box_dimensions[0] as f32 / cell_size).ceil() as i32;
    let number_of_rows = (box_dimensions[1] as f32 / cell_size).ceil() as i32;
    let number_of_cells = (number_of_columns * number_of_rows) as i32;
    CellManager {
      particle_count,
      spatial_lookup: (0..particle_count).map(|_| (number_of_cells as usize, 0)).collect(),
      starting_indices: (0..number_of_cells).map(|_| number_of_cells as usize).collect(),
      number_of_columns,
      number_of_rows,
      cell_size,
      number_of_cells
    }
  }

  pub fn update(&mut self, particles: &mut Vec<Particle>) {
    for particle in particles { 
      self.to_spacial_lookup(particle)
    }
    self.spatial_lookup.sort_by(|s_a, s_b| s_a.0.cmp(&s_b.0));
    self.generate_start_indices();
  }

  pub fn get_adjancet_particles(&self, particle: Particle, particles: &Vec<Particle>) -> Vec<Particle> {
    let mut adjacent_particle_indexes: Vec<usize> = vec![0];
    let adjacent_cell_keys = self.get_adjacent_cell_keys_from_position(particle.position);
    for adjacent_cell_key in adjacent_cell_keys {
      let particle_indexes = self.get_particle_indexes_from_cell(adjacent_cell_key);
      adjacent_particle_indexes.extend(particle_indexes)
    }
    adjacent_particle_indexes.iter().map(|index| particles[*index].clone()).collect()
  }

  fn to_spacial_lookup(&mut self, particle: &mut Particle) {
    let cell_coord = self.particle_position_to_cell_coord(particle.position);
    let cell_key = self.cell_coord_to_cell_key(cell_coord);
    particle.cell_key = cell_key;
    self.spatial_lookup[particle.id as usize] = (cell_key, particle.id)
  }

  fn generate_start_indices(&mut self) {
    let mut starting_indices: Vec<usize> = (0..self.number_of_cells).map(|_| self.number_of_cells as usize).collect();
    let mut current_cell_key = self.spatial_lookup[0].0;
    for (spatial_lookup_index, spatial_lookup) in self.spatial_lookup.iter().enumerate() {
      if spatial_lookup.0 == current_cell_key { continue }
      current_cell_key = spatial_lookup.0;
      starting_indices[current_cell_key] = spatial_lookup_index
    }
    self.starting_indices = starting_indices;
  }

  fn get_adjacent_cell_keys_from_position(&self, position: Vector2D<f32>) -> Vec<usize> {
    let current_cell_coord = self.particle_position_to_cell_coord(position);
    let mut adjacent_cell_coords = vec![
        current_cell_coord,
        current_cell_coord + Vector2D::new(1, 0),
        current_cell_coord + Vector2D::new(-1, 0),
        current_cell_coord + Vector2D::new(0, 1),
        current_cell_coord + Vector2D::new(1, -1),
        current_cell_coord + Vector2D::new(1, 1),
        current_cell_coord + Vector2D::new(1, -1),
        current_cell_coord + Vector2D::new(-1, 1),
        current_cell_coord + Vector2D::new(-1, -1),
    ];
    
    adjacent_cell_coords = adjacent_cell_coords
        .iter()
        .cloned() // Clone the elements to create owned copies
        .filter(|coord| coord.x > 0 && coord.x < self.number_of_columns && coord.y > 0 && coord.y < self.number_of_rows)
        .collect();
    adjacent_cell_coords.iter().map(|coord| self.cell_coord_to_cell_key(*coord)).collect()
  }

  fn get_particle_indexes_from_cell(&self, cell_key: usize) -> Vec<usize> {
    let mut particle_indexes: Vec<usize> = Vec::new();
    let mut spatial_lookup_cell: usize = cell_key;
    let mut spatial_lookup_index = self.starting_indices[cell_key];
    if spatial_lookup_index >= self.particle_count as usize {
      return particle_indexes;
    }
    while cell_key == spatial_lookup_cell && spatial_lookup_index < self.particle_count as usize {
      let particle_index = self.spatial_lookup[spatial_lookup_index].1;
      particle_indexes.push(particle_index);
      spatial_lookup_index += 1;
      spatial_lookup_cell = self.starting_indices[cell_key];
    }
    particle_indexes
  }

  pub fn particle_position_to_cell_coord(&self, position: Vector2D<f32>) -> Vector2D<i32> {
    let x = (position.x / self.cell_size).floor() as i32;
    let y = (position.y / self.cell_size).floor() as i32;
    Vector2D::new(x, y)
  }

  pub fn cell_coord_to_cell_key(&self, coord: Vector2D<i32>) -> usize {
    ((coord.x * self.number_of_columns) + coord.y) as usize
  }
}