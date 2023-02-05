use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::glam::*;

mod util;
pub use crate::util::*;

const WIN_WIDTH: f32 = 1440.0;
const WIN_HEIGHT: f32 = 960.0;

fn main() {
    // Make a Context.
    let cb = ContextBuilder::new("3D Rendered", "Ruben Saunders")
        .window_setup(WindowSetup::default().title("3D Renderer"))
        .window_mode(WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT));
    let (mut ctx, event_loop) = cb.build().expect("Could not create ggez context!");
    let rd = Renderer::new(&mut ctx);

    event::run(ctx, event_loop, rd);
}

struct Renderer {
    mesh: Mesh,
    proj: Mat4x4,
    pos: Vec3d,
    theta: f64,
}

impl Renderer {
    pub fn new(_ctx: &mut Context) -> Renderer {
        let near = 1f64;
        let far = 1000f64;
        let fov = 90f64;
        let fov_rad = 1f64 / (fov * 0.5f64 / 180f64 * std::f64::consts::PI).tan();
        let ar = (WIN_HEIGHT / WIN_WIDTH) as f64;
        Renderer {
            mesh: Mesh::new_unit_cube(),
            proj: Mat4x4(
                (ar * fov_rad, 0.0, 0.0, 0.0),
                (0.0, fov_rad, 0.0, 0.0),
                (0.0, 0.0, far / (far - near), 1.0),
                (0.0, 0.0, (-far * near) / (far - near), 0.0),
            ),
            pos: Vec3d::new(0.0, 0.0, 3.0),
            theta: 0.0,
        }
    }
}

impl EventHandler for Renderer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.theta = self.theta + 0.02; // % (2.0 * std::f64::consts::PI);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rot_x = Mat4x4::rot_x(self.theta * 0.5);
        let rot_z = Mat4x4::rot_z(self.theta);
        let rot = Mat4x4::mult(&rot_z, &rot_x);
        let mb = &mut graphics::MeshBuilder::new();

        // Project triangles
        for tri in &self.mesh.tris {
            // Rotate
            let mut tri_trans = Triangle::new(
                Vec3d::mult_mat(&tri.vertices.0, &rot),
                Vec3d::mult_mat(&tri.vertices.1, &rot),
                Vec3d::mult_mat(&tri.vertices.2, &rot),
            );

            // Translate
            tri_trans.vertices.0.add(&self.pos);
            tri_trans.vertices.1.add(&self.pos);
            tri_trans.vertices.2.add(&self.pos);

            let mut tri_proj = Triangle::new(
                Vec3d::mult_mat(&tri_trans.vertices.0, &self.proj),
                Vec3d::mult_mat(&tri_trans.vertices.1, &self.proj),
                Vec3d::mult_mat(&tri_trans.vertices.2, &self.proj),
            );

            tri_proj.vertices.0.addk(1.0);
            tri_proj.vertices.1.addk(1.0);
            tri_proj.vertices.2.addk(1.0);

            tri_proj.vertices.0.x *= 0.5 * WIN_WIDTH as f64;
            tri_proj.vertices.0.y *= 0.5 * WIN_HEIGHT as f64;
            tri_proj.vertices.1.x *= 0.5 * WIN_WIDTH as f64;
            tri_proj.vertices.1.y *= 0.5 * WIN_HEIGHT as f64;
            tri_proj.vertices.2.x *= 0.5 * WIN_WIDTH as f64;
            tri_proj.vertices.2.y *= 0.5 * WIN_HEIGHT as f64;

            let vs = [
                vec2(tri_proj.vertices.0.x as f32, tri_proj.vertices.0.y as f32),
                vec2(tri_proj.vertices.1.x as f32, tri_proj.vertices.1.y as f32),
                vec2(tri_proj.vertices.2.x as f32, tri_proj.vertices.2.y as f32),
                vec2(tri_proj.vertices.0.x as f32, tri_proj.vertices.0.y as f32),
            ];

            mb.line(
                &vs,
                2.0,
                Color::WHITE,
            )?;
        }

        let mesh = graphics::Mesh::from_data(ctx, mb.build());

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&mesh, graphics::DrawParam::default());
        canvas.finish(ctx)?;

        Ok(())
    }
}

