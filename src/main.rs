use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::glam::*;

mod util;
pub use util::mesh::{Mesh, shape};
pub use util::maths::mat::Mat4x4;
pub use util::maths::vec::Vec3d;
use util::triangle::Triangle;

const WIN_WIDTH: f32 = 1090.0;
const WIN_HEIGHT: f32 = 720.0;

fn main() {
    // Make a Context.
    let cb = ContextBuilder::new("3D Renderer", "triflicAcid")
        .window_setup(WindowSetup::default().title("3D Renderer"))
        .window_mode(WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT));
    let (mut ctx, event_loop) = cb.build().expect("Could not create ggez context!");
    let mut rd = Renderer::new(&mut ctx);
    

    let style = util::mesh::FaceSettings {
        fill: Some(colorsys::Rgb::new(255.0, 255.0, 128.0, None)),
        // stroke: Some(colorsys::Rgb::new(255.0, 0.0, 0.0, None)),
        stroke: None,
    };

    let res = util::mesh::obj::parse_file("models/spaceship.obj".to_string());
    if res.is_err() {
        println!("{}", res.unwrap_err());
    } else {
        let mut mesh = res.unwrap();

        mesh.add_style(&style);
        mesh.set_global_style(Some(0));
        rd.mesh.merge(&mut mesh);
    }


    event::run(ctx, event_loop, rd);
}

struct Renderer {
    mesh: Mesh,
    proj: Mat4x4,
    theta: f32,
    light_dir: Vec3d,
    camera: Vec3d,
}

impl Renderer {
    pub fn new(_ctx: &mut Context) -> Renderer {
        let near = 1.0;
        let far = 1000.0;
        let fov = 90.0;
        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();
        let ar = WIN_HEIGHT / WIN_WIDTH;
        Renderer {
            mesh: Mesh::new(),
            proj: Mat4x4(
                (ar * fov_rad, 0.0, 0.0, 0.0),
                (0.0, fov_rad, 0.0, 0.0),
                (0.0, 0.0, far / (far - near), 1.0),
                (0.0, 0.0, (-far * near) / (far - near), 0.0),
            ),
            light_dir: Vec3d::new(0.0, 0.0, -1.0),
            camera: Vec3d::new(0.0, 0.0, 0.0),
            theta: 0.0,
        }
    }
}

impl EventHandler for Renderer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.theta = self.theta + 0.02; // % (2.0 * std::f32::consts::PI);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rot_x = Mat4x4::rot_x(self.theta * 0.5);
        let rot_z = Mat4x4::rot_z(self.theta);
        let rot = Mat4x4::mult(&rot_z, &rot_x);
        let scale = Vec3d::new(0.5 * WIN_WIDTH, 0.5 * WIN_HEIGHT, 1.0);
        let light_dir = self.light_dir.normalise();
        let mb = &mut graphics::MeshBuilder::new();
        let mut to_draw: Vec<Triangle> = Vec::new();

        for i in 0..self.mesh.face_count() {
            let mut tri = self.mesh.compile_face(i).unwrap();

            // Rotate and translate
            tri.vertices.0 = Vec3d::mult_mat(&tri.vertices.0, &rot);
            tri.vertices.1 = Vec3d::mult_mat(&tri.vertices.1, &rot);
            tri.vertices.2 = Vec3d::mult_mat(&tri.vertices.2, &rot);

            // Offset
            tri.vertices.0.z += 5.0;
            tri.vertices.1.z += 5.0;
            tri.vertices.2.z += 5.0;

            // Calculate the normal
            let normal = tri.normal().normalise();

            // Check if triangle is visible - less than 90deg to the camera
            let cam_ray = tri.vertices.0.sub(&self.camera);
            if Vec3d::dot_product(&normal, &cam_ray) < 0.0 {
                // Illumination
                let lum = Vec3d::dot_product(&normal, &light_dir).max(0.1) as f64;

                // Project the triangle
                tri.vertices.0 = Vec3d::mult_mat(&tri.vertices.0, &self.proj);
                tri.vertices.1 = Vec3d::mult_mat(&tri.vertices.1, &self.proj);
                tri.vertices.2 = Vec3d::mult_mat(&tri.vertices.2, &self.proj);
    
                tri.vertices.0 = tri.vertices.0.addk(1.0).mul(&scale);
                tri.vertices.1 = tri.vertices.1.addk(1.0).mul(&scale);
                tri.vertices.2 = tri.vertices.2.addk(1.0).mul(&scale);

                // Add to vector to draw
                tri.lum = lum;
                to_draw.push(tri);
            }
        }

        // Sort by Z component
        to_draw.sort_by(|a, b| b.mid_z().partial_cmp(&a.mid_z()).unwrap());

        // Draw triangles
        for i in 0..to_draw.len() {
            let tri = &mut to_draw[i];

            // Get points
            let vs = [
                vec2(tri.vertices.0.x as f32, tri.vertices.0.y as f32),
                vec2(tri.vertices.1.x as f32, tri.vertices.1.y as f32),
                vec2(tri.vertices.2.x as f32, tri.vertices.2.y as f32),
                vec2(tri.vertices.0.x as f32, tri.vertices.0.y as f32),
            ];

            // Fill
            if tri.fill.is_some() {
                // Apply brightness
                let rgb = tri.get_fill().unwrap();

                mb.polyline(
                    graphics::DrawMode::fill(),
                    &vs,
                    // graphics::Color::new(lum, lum, lum, 1.0),
                    Color::new((rgb.red() / 255.0) as f32, (rgb.green() / 255.0) as f32, (rgb.blue() / 255.0) as f32, 1.0),
                )?;
            }

            // Stroke
            if tri.stroke.is_some() {
                let rgb = tri.get_stroke().unwrap();
                mb.polyline(
                    graphics::DrawMode::stroke(2.0),
                    &vs,
                    Color::new((rgb.red() / 255.0) as f32, (rgb.green() / 255.0) as f32, (rgb.blue() / 255.0) as f32, 1.0),
                )?;
            }
        }

        let mesh = graphics::Mesh::from_data(ctx, mb.build());

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&mesh, graphics::DrawParam::default());
        canvas.finish(ctx)?;

        Ok(())
    }
}

