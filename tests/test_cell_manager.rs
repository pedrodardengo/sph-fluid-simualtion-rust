#[cfg(test)]
mod tests {
    use coding_challenges::fluid_simulation::{cell_manager::CellManager, particle::Particle};
    use vector2d::Vector2D;

    #[test]
    fn test_update_cell_keys() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [usize; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let particles: &mut [Particle; 9] = &mut [
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5)),
        ];
        let mut cell_manager =
            CellManager::new(particle_count as i32, box_dimensions, smoothing_radius);

        // ACT
        cell_manager.update(particles);

        // ASSERT
        for index in 0..8 {
            assert_eq!(particles[index].cell_key, index);
        }
    }

    #[test]
    fn test_get_adjancet_particles() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [usize; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let particles: &mut [Particle; 9] = &mut [
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5)),
        ];
        let mut cell_manager =
            CellManager::new(particle_count as i32, box_dimensions, smoothing_radius);
        cell_manager.update(particles);

        // ACT
        let adjacent_particles_indices_iterator =
            cell_manager.get_adjacent_particles_indices(particles[4].position);

        // ASSERT
        let adjacent_particles_indices: Vec<usize> = adjacent_particles_indices_iterator.collect();
        assert_eq!(adjacent_particles_indices.len(), 9);
    }
}
