use crate::fluid_simulation::config::Particles;
use crate::fluid_simulation::particle::Particle;
use graphics::rectangle::rectangle_by_corners;
use graphics::{
    math::{Matrix2d, Vec2d},
    triangulation::{tx, ty},
    *,
};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct RenderManager {
    gl: GlGraphics,
}

fn stream_polygon_tri_list<E, F>(m: Matrix2d, mut polygon: E, mut f: F)
where
    E: Iterator<Item = Vec2d>,
    F: FnMut(&[[f32; 2]]),
{
    let mut vertices: [[f32; 2]; 20000] = [[0.0; 2]; 20000];
    let mut i = 0;
    'read_vertices: loop {
        let p = match polygon.next() {
            None => break 'read_vertices,
            Some(val) => val,
        };
        let ind_out = i;
        vertices[ind_out] = [tx(m, p[0], p[1]), ty(m, p[0], p[1])];
        i += 1;
        // Buffer is full.
        if (i + 1) > 20000 {
            // Send chunk and start over.
            f(&vertices[0..i]);
            i = 0;
        }
    }

    if i > 0 {
        f(&vertices[0..i]);
    }
}

fn with_polygon_tri_list<F>(m: Matrix2d, polygon: &[[f64; 2]], f: F)
where
    F: FnMut(&[[f32; 2]]),
{
    stream_polygon_tri_list(m, (0..polygon.len()).map(|i| polygon[i]), f);
}

impl RenderManager {
    pub fn new(gl: GlGraphics) -> Self {
        RenderManager { gl }
    }

    pub fn render_obstacles(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            let dims = rectangle_by_corners(50.0, 300.0, 100.0, 500.0);
            rectangle([0.3, 0.3, 0.3, 1.0], dims, c.transform, gl);
        })
    }

    pub fn render(&mut self, args: &RenderArgs, particles: &Particles) {
        const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            let dims = rectangle_by_corners(100.0, 400.0, 200.0, 750.0);
            rectangle([0.3, 0.3, 0.3, 1.0], dims, c.transform, gl);

            clear(BLACK_COLOR, gl);
            let verts = particles
                .iter()
                .map(|particle| {
                    [
                        [particle.position.x as f64, particle.position.y as f64 + 3.0],
                        [
                            particle.position.x as f64 + 3.0,
                            particle.position.y as f64 - 3.0,
                        ],
                        [
                            particle.position.x as f64 - 3.0,
                            particle.position.y as f64 - 3.0,
                        ],
                    ]
                })
                .flatten()
                .collect::<Vec<_>>();

            let colors = particles
                .iter()
                .map(|particle| {
                    let speed = speed_to_color_gradient(particle.speed());
                    [speed, speed, speed]
                })
                .flatten()
                .collect::<Vec<_>>();

            gl.tri_list_c(&DrawState::default(), |f| {
                with_polygon_tri_list(c.transform, verts.as_slice(), |vertices| {
                    f(vertices, colors.as_slice())
                })
            });
        });
    }
}

const INVERSED_MAX_SPEED: f32 = 1.0 / 800.0;
const INVERSE_255: f32 = 1.0 / 255.0;
fn speed_to_color_gradient(speed: f32) -> [f32; 4] {
    let ratio: f32 = speed * INVERSED_MAX_SPEED;
    let normalized = (ratio * 1024.0) as i32;
    let region = (normalized / 256) as i32;
    let x = (normalized % 256) as f32;
    match region {
        3 => [1.0, f32::max(255.0 - x, 0.0) * INVERSE_255, 0.0, 1.0],
        2 => [f32::max(x, 0.0) * INVERSE_255, 1.0, 0.0, 1.0],
        1 => [0.0, 1.0, f32::max(255.0 - x, 0.0) * INVERSE_255, 1.0],
        0 => [0.0, f32::max(x, 0.0) * INVERSE_255, 1.0, 1.0],
        _ => [1.0, 0.0, 0.0, 1.0],
    }
}
