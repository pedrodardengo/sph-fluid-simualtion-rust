use crate::fluid_simulation::config::{Particles, PARTICLE_COUNT};
use crate::fluid_simulation::particle::Particle;
use vector2d::Vector2D;

type SpatialLookup = [(usize, usize); PARTICLE_COUNT];
pub struct CellManager {
    particle_count: i32,
    spatial_lookup: SpatialLookup,
    starting_indices: Vec<usize>,
    number_of_columns: i32,
    number_of_rows: i32,
    cell_size: f32,
    number_of_cells: i32,
}

impl CellManager {
    pub fn new(particle_count: i32, box_dimensions: [usize; 2], smoothing_radius: f32) -> Self {
        let cell_size = 2.0 * smoothing_radius;
        let number_of_columns = (box_dimensions[0] as f32 / cell_size).ceil() as i32;
        let number_of_rows = (box_dimensions[1] as f32 / cell_size).ceil() as i32;
        let number_of_cells = (number_of_columns * number_of_rows) as i32;
        CellManager {
            particle_count,
            spatial_lookup: core::array::from_fn(|_| (number_of_cells as usize, 0)),
            starting_indices: (0..number_of_cells)
                .map(|_| number_of_cells as usize)
                .collect(),
            number_of_columns,
            number_of_rows,
            cell_size,
            number_of_cells,
        }
    }

    pub fn update(&mut self, particles: &mut Particles) {
        for particle in particles {
            self.to_spacial_lookup(particle)
        }
        self.spatial_lookup.sort_by(|s_a, s_b| s_a.0.cmp(&s_b.0));
        self.generate_start_indices();
    }

    pub fn get_adjacent_particles_indices<'a>(
        &'a self,
        particle_position: Vector2D<f32>,
    ) -> impl Iterator<Item = usize> + 'a {
        self.get_adjacent_cell_keys_from_position(particle_position)
            .flat_map(|adjacent_cell_key| self.get_particle_indexes_from_cell(adjacent_cell_key))
    }

    fn to_spacial_lookup(&mut self, particle: &mut Particle) {
        let cell_coord = self.particle_position_to_cell_coord(particle.position);
        let cell_key = self.cell_coord_to_cell_key(cell_coord);
        particle.cell_key = cell_key;
        self.spatial_lookup[particle.id as usize] = (cell_key, particle.id)
    }

    fn generate_start_indices(&mut self) {
        self.starting_indices = vec![self.particle_count as usize; self.number_of_cells as usize];
        self.spatial_lookup
            .iter()
            .enumerate()
            .for_each(|(sl_index, &(cell_key, _))| {
                if self.starting_indices[cell_key] == self.particle_count as usize {
                    self.starting_indices[cell_key] = sl_index;
                }
            });
    }

    fn get_adjacent_cell_keys_from_position<'a>(
        &'a self,
        position: Vector2D<f32>,
    ) -> impl Iterator<Item = usize> + 'a {
        let current_cell_coord = self.particle_position_to_cell_coord(position);
        let adjacent_cell_coords = vec![
            current_cell_coord + Vector2D::new(-1, -1),
            current_cell_coord + Vector2D::new(-1, 0),
            current_cell_coord + Vector2D::new(-1, 1),
            current_cell_coord + Vector2D::new(0, -1),
            current_cell_coord,
            current_cell_coord + Vector2D::new(0, 1),
            current_cell_coord + Vector2D::new(1, -1),
            current_cell_coord + Vector2D::new(1, 0),
            current_cell_coord + Vector2D::new(1, 1),
        ];
        adjacent_cell_coords
            .into_iter()
            .filter(|coord| {
                coord.x >= 0
                    && coord.x < self.number_of_columns
                    && coord.y >= 0
                    && coord.y < self.number_of_rows
            })
            .map(|coord| self.cell_coord_to_cell_key(coord))
    }

    fn get_particle_indexes_from_cell(&self, cell_key: usize) -> Vec<usize> {
        let mut particle_indexes: Vec<usize> = Vec::new();
        let mut spatial_lookup_cell: usize = cell_key;
        let mut spatial_lookup_index = self.starting_indices[cell_key];
        if spatial_lookup_index >= self.particle_count as usize {
            return particle_indexes;
        }
        while cell_key == spatial_lookup_cell {
            let particle_index = self.spatial_lookup[spatial_lookup_index].1;
            particle_indexes.push(particle_index);
            spatial_lookup_index += 1;
            if spatial_lookup_index >= self.particle_count as usize {
                break;
            }
            spatial_lookup_cell = self.spatial_lookup[spatial_lookup_index].0;
        }
        particle_indexes
    }

    fn particle_position_to_cell_coord(&self, position: Vector2D<f32>) -> Vector2D<i32> {
        let x = (position.x / self.cell_size).floor() as i32;
        let y = (position.y / self.cell_size).floor() as i32;
        Vector2D::new(x, y)
    }

    fn cell_coord_to_cell_key(&self, coord: Vector2D<i32>) -> usize {
        ((coord.x * self.number_of_rows) + coord.y) as usize
    }
}
