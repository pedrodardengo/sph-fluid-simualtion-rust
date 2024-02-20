

#[cfg(test)]
mod tests {
    use coding_challenges::fluid_simulation::{cell_manager::CellManager, particle::Particle};
    use vector2d::Vector2D;

    #[test]
    fn test_update_cell_keys() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [i32; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let particles: &mut Vec<Particle> = &mut vec![
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5))
        ];
        let mut cell_manager = CellManager::new(
            particle_count as i32,
            box_dimensions,
            smoothing_radius
        );

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
        let box_dimensions: [i32; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let particles: &mut Vec<Particle> = &mut vec![
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5))
        ];
        let mut cell_manager = CellManager::new(
            particle_count as i32,
            box_dimensions,
            smoothing_radius
        );
        cell_manager.update(particles);

        // ACT
        let adjacent_particles = cell_manager.get_adjancet_particles(particles[4].clone(), particles);

        // ASSERT
        assert_eq!(adjacent_particles.len(), 9);

    }


    #[test]
    fn test_particle_position_to_cell_coord() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [i32; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let cell_manager = CellManager::new(
            particle_count as i32,
            box_dimensions,
            smoothing_radius
        );
        let particles: &mut Vec<Particle> = &mut vec![
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5))
        ];

        // ACT
        let coord_0 = cell_manager.particle_position_to_cell_coord(particles[0].position);
        let coord_1 = cell_manager.particle_position_to_cell_coord(particles[1].position);
        let coord_2 = cell_manager.particle_position_to_cell_coord(particles[2].position);
        let coord_3 = cell_manager.particle_position_to_cell_coord(particles[3].position);
        let coord_4 = cell_manager.particle_position_to_cell_coord(particles[4].position);
        let coord_5 = cell_manager.particle_position_to_cell_coord(particles[5].position);
        let coord_6 = cell_manager.particle_position_to_cell_coord(particles[6].position);
        let coord_7 = cell_manager.particle_position_to_cell_coord(particles[7].position);
        let coord_8 = cell_manager.particle_position_to_cell_coord(particles[8].position);

        // ASSERT
        assert_eq!([coord_0.x, coord_0.y], [0, 0]);
        assert_eq!([coord_1.x, coord_1.y], [0, 1]);
        assert_eq!([coord_2.x, coord_2.y], [0, 2]);
        assert_eq!([coord_3.x, coord_3.y], [1, 0]);
        assert_eq!([coord_4.x, coord_4.y], [1, 1]);
        assert_eq!([coord_5.x, coord_5.y], [1, 2]);
        assert_eq!([coord_6.x, coord_6.y], [2, 0]);
        assert_eq!([coord_7.x, coord_7.y], [2, 1]);
        assert_eq!([coord_8.x, coord_8.y], [2, 2]);
    }

    #[test]
    fn test_cell_coord_to_cell_key() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [i32; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let cell_manager = CellManager::new(
            particle_count as i32,
            box_dimensions,
            smoothing_radius
        );

        // ACT
        let cell_key_0 = cell_manager.cell_coord_to_cell_key(Vector2D::new(0, 0));
        let cell_key_1 = cell_manager.cell_coord_to_cell_key(Vector2D::new(0, 1));
        let cell_key_2 = cell_manager.cell_coord_to_cell_key(Vector2D::new(0, 2));
        let cell_key_3 = cell_manager.cell_coord_to_cell_key(Vector2D::new(1, 0));
        let cell_key_4 = cell_manager.cell_coord_to_cell_key(Vector2D::new(1, 1));
        let cell_key_5 = cell_manager.cell_coord_to_cell_key(Vector2D::new(1, 2));
        let cell_key_6 = cell_manager.cell_coord_to_cell_key(Vector2D::new(2, 0));
        let cell_key_7 = cell_manager.cell_coord_to_cell_key(Vector2D::new(2, 1));
        let cell_key_8 = cell_manager.cell_coord_to_cell_key(Vector2D::new(2, 2));

        // ASSERT
        assert_eq!(cell_key_0, 0);
        assert_eq!(cell_key_1, 1);
        assert_eq!(cell_key_2, 2);
        assert_eq!(cell_key_3, 3);
        assert_eq!(cell_key_4, 4);
        assert_eq!(cell_key_5, 5);
        assert_eq!(cell_key_6, 6);
        assert_eq!(cell_key_7, 7);
        assert_eq!(cell_key_8, 8);
    }

    #[test]
    fn test_get_ajacent_cell_keys() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [i32; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let particles: &mut Vec<Particle> = &mut vec![
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5))
        ];
        let mut cell_manager = CellManager::new(
            particle_count as i32,
            box_dimensions,
            smoothing_radius
        );
        cell_manager.update(particles);

        // ACT
        let cell_keys = cell_manager.get_adjacent_cell_keys_from_position(particles[4].position);

        // ASSERT
        for (index, cell_key) in cell_keys.iter().enumerate() {
            assert_eq!(cell_key, &index);
        }
    }

    #[test]
    fn test_get_particle_indexes_from_cell() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [i32; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let particles: &mut Vec<Particle> = &mut vec![
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5))
        ];
        let mut cell_manager = CellManager::new(
            particle_count as i32,
            box_dimensions,
            smoothing_radius
        );
        cell_manager.update(particles);

        // ACT
        for index in 0..8 {
            let particle_indexes = cell_manager.get_particle_indexes_from_cell(particles[index].cell_key);

            // ASSERT
            assert_eq!(particle_indexes[0], index);
            assert_eq!(particle_indexes.len(), 1);
        }
    }

    #[test]
    fn test_start_indices_generation() {
        // ARRANGE
        let particle_count: usize = 9;
        let box_dimensions: [i32; 2] = [3, 3];
        let smoothing_radius: f32 = 0.5;
        let particles: &mut Vec<Particle> = &mut vec![
            Particle::new(0, Vector2D::new(0.5, 0.5)),
            Particle::new(1, Vector2D::new(0.5, 1.5)),
            Particle::new(2, Vector2D::new(0.5, 2.5)),
            Particle::new(3, Vector2D::new(1.5, 0.5)),
            Particle::new(4, Vector2D::new(1.5, 1.5)),
            Particle::new(5, Vector2D::new(1.5, 2.5)),
            Particle::new(6, Vector2D::new(2.5, 0.5)),
            Particle::new(7, Vector2D::new(2.5, 1.5)),
            Particle::new(8, Vector2D::new(2.5, 2.5))
        ];
        let mut cell_manager = CellManager::new(
            particle_count as i32,
            box_dimensions,
            smoothing_radius
        );
        cell_manager.update(particles);

        // ACT
        assert_eq!(cell_manager.starting_indices, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}